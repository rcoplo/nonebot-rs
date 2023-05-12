use async_trait::async_trait;
use tracing::{event, Level};
use crate::event::{Event, GroupId, NoticeEvent, NoticeSubType, NoticeType, Role, UserId};
use crate::matcher::{build_temp_notice_event_matcher, Handler, Matcher};
use colored::Colorize;
use crate::ApiChannelItem;

impl Matcher<NoticeEvent> {
    /// 发送纯文本消息
    pub async fn send_text_(&self, msg: &str) -> Option<crate::api_resp::MessageId> {
        let msg = crate::message::Message::Text {
            text: msg.to_string(),
        };
        self.send(vec![msg]).await
    }
    /// 直接发送纯文本消息
    pub async fn send_text(&self, msg: &str) {
        let msg = crate::message::Message::Text {
            text: msg.to_string(),
        };
        self.send_(vec![msg]).await;
    }
    
    /// 发送 Vec<Message> 消息 带 message_id
    pub async fn send(&self, msg: crate::message::MessageChain) -> Option<crate::api_resp::MessageId> {
        if let (Some(bot), Some(event)) = (&self.bot, &self.event) {
            bot.send_by_notice_event_(event, msg).await
        } else {
            event!(
                Level::ERROR,
                "{}",
                "Sending msg with unbuilt matcher!".red()
            );
            None
        }
    }
    
    /// 发送 Vec<Message> 消息 直接 发送,不带返回值
    pub async fn send_(&self, msg: crate::message::MessageChain) {
        if let (Some(bot), Some(event)) = (&self.bot, &self.event) {
            bot.send_by_notice_event(event, msg).await;
        } else {
            event!(
                Level::ERROR,
                "{}",
                "Sending msg with unbuilt matcher!".red()
            );
        }
    }
    /// 戳(
    pub async fn poke(&self) {
        if let Some(event) = &self.event {
            if let Some(sub_type) = &event.sub_type {
                match sub_type {
                    NoticeSubType::Poke => {
                        let qq = crate::message::Message::Poke {
                            qq: event.user_id,
                        };
                        self.send_(vec![qq]).await;
                    }
                    _ => {}
                }
            }
        } else {
            event!(
                Level::ERROR,
                "{}",
                "poke failed".red()
            );
        }
    }
    /// 是否是管理员或群主 私聊则返回false
    pub async fn is_admin(&self) -> bool {
        if let (Some(bot), Some(event)) = (&self.bot, &self.event) {
            if !bot.config.superusers.is_empty() {
                let user_id = event.get_user_id().to_string();
                for x in &bot.config.superusers {
                    if x.eq(&user_id) {
                        return true;
                    }
                }
            }
            let (user_id, group_id) = (event.get_user_id(), event.get_group_id());
            match bot.get_group_member_list(group_id).await {
                None => return false,
                Some(v) => {
                    for x in v {
                        if x.user_id.eq(&user_id) {
                            return match x.role {
                                Role::Owner => true,
                                Role::Admin => true,
                                Role::Member => false,
                            };
                        }
                    }
                }
            };
        } else {
            event!(
                Level::ERROR,
                "{}",
                "admin acquisition failed!".red()
            );
        }
        false
    }
    /// 设置临时 Matcher<MessageEvent>
    pub async fn set_temp_notice_event_matcher<H>(&self, event: &NoticeEvent, handler: H)
        where
            H: Handler<NoticeEvent> + Send + Sync + 'static,
    {
        self.set_notice_matcher(build_temp_notice_event_matcher(None, 30, event, handler))
            .await;
    }
    
    pub async fn request_notice_callback(
        &self,
        timeout: i64,
        user_id: Option<i64>,
        notice_type: Option<NoticeType>,
        sub_type: Option<NoticeSubType>,
        msg: Option<&str>,
    ) -> Option<NoticeEvent> {
        /// 临时 Matcher 的 Handler struct
        struct TempNoticeMatcher;
        
        #[async_trait]
        impl Handler<NoticeEvent> for TempNoticeMatcher {
            // timeout 后调用，通知接受端 Timeout
            fn timeout_drop(&self, matcher: &Matcher<NoticeEvent>) {
                let sender = matcher.bot.clone().unwrap().api_sender;
                tokio::spawn(async move { sender.send(ApiChannelItem::TimeOut).await.unwrap() });
            }
            
            fn match_(&mut self, _: &mut NoticeEvent) -> bool {
                true
            }
            
            async fn handle(&self, event: NoticeEvent, matcher: &mut Matcher<NoticeEvent>) {
                matcher
                    .bot
                    .clone()
                    .unwrap()
                    .api_sender
                    .send(ApiChannelItem::NoticeEvent(event))
                    .await
                    .unwrap();
            }
        }
        
        // 搭建临时通道接受 MessageEvent
        let (sender, mut receiver) = tokio::sync::mpsc::channel::<ApiChannelItem>(4);
        let event = self.event.clone().unwrap();
        // 根据提供的 event Handler 构建仅指向当先通话的 Temp Matcher
        let mut m = build_temp_notice_event_matcher(user_id, timeout, &event, TempNoticeMatcher);
        // 使用临时通道构建专用 Bot
        let bot = crate::bot::Bot::new(
            0,
            crate::config::BotConfig::default(),
            sender,
            self.bot.clone().unwrap().action_sender.clone(),
            self.bot.clone().unwrap().api_resp_watcher.clone(),
        );
        // 绑定专用 Bot
        m.bot = Some(bot);
        self.set_notice_matcher(m).await;
        
        // Temp Matcher 已就绪，发送提示信息
        if let Some(msg) = msg {
            self.send_text(msg).await;
        }
        
        // 等待接收 MessageEvent
        while let Some(data) = receiver.recv().await {
            match data {
                ApiChannelItem::NoticeEvent(event) => {
                    if notice_type.is_none() && sub_type.is_none() {
                        return Some(event);
                    }
                    
                    if let Some(n) = &notice_type {
                        if &event.notice_type == n {
                            return Some(event);
                        }
                    }
                    
                    if let Some(n) = &sub_type {
                        if let Some(s) = &event.sub_type {
                            if s == n {
                                return Some(event);
                            }
                        }
                    }
                    
                    return None;
                }
                ApiChannelItem::TimeOut => {
                    event!(Level::DEBUG, "Temp Notice Matcher TimeOut");
                    return None;
                }
                // 中转 temp Matcher 的 Remove Action
                // ApiChannelItem::Action(action) => self.set(action).await,
                _ => {
                    event!(
                        Level::WARN,
                        "{}",
                        "Temp Notice Matcher 接受端接收到错误Api或Action消息".bright_red()
                    );
                } // 忽视 event 该 receiver 永不应该收到 event
            }
        }
        
        None
    }
}