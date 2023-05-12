use std::future::Future;
use std::sync::Arc;
use super::{build_temp_message_event_matcher, Handler, Matcher};
use crate::event::{MessageEvent, NoticeEvent, NoticeSubType, NoticeType, Role, UserId};
use crate::{ApiChannelItem, NBError, NBResult};
use async_trait::async_trait;
use colored::*;
use futures_util::SinkExt;
use tracing::{event, Level};
use crate::matcher::{build_temp_notice_event_matcher, Session};
use crate::message::MessageChain;
use crate::utils::remove_space;

impl Matcher<MessageEvent> {
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
    /// 直接发送带回复的纯文本消息
    pub async fn reply_text(&self, msg: &str) {
        if let Some(message_id) = self._reply_text().await {
            self.send_(vec![
                crate::message::Message::Reply {
                    id: message_id,
                    text: None,
                    qq: None,
                    time: None,
                    seq: None,
                }, crate::message::Message::Text {
                    text: msg.to_string(),
                }]).await;
        }
    }
    /// 直接发送带At的纯文本消息
    pub async fn at_text(&self, msg: &str) {
        if let Some(user_id) = self._at_text().await {
            self.send_(vec![
                crate::message::Message::At {
                    qq: user_id.to_string(),
                    name: None,
                }, crate::message::Message::Text {
                    text: msg.to_string(),
                }]).await;
        }
    }
    /// 发送带回复纯文本消息
    pub async fn reply_text_(&self, msg: &str) -> Option<crate::api_resp::MessageId> {
        if let Some(message_id) = self._reply_text().await {
            return self.send(vec![
                crate::message::Message::Reply {
                    id: message_id,
                    text: None,
                    qq: None,
                    time: None,
                    seq: None,
                }, crate::message::Message::Text {
                    text: msg.to_string(),
                }]).await;
        }
        None
    }

    async fn _reply_text(&self) -> Option<i32> {
        if let Some(event) = &self.event {
            let message_id = match event {
                MessageEvent::Private(p) => {
                    p.message_id
                }
                MessageEvent::Group(e) => {
                    e.message_id
                }
            };
            return Some(message_id);
        }
        None
    }
    /// 发送带At纯文本消息
    pub async fn at_text_(&self, msg: &str) -> Option<crate::api_resp::MessageId> {
        if let Some(user_id) = self._at_text().await {
            return self.send(vec![
                crate::message::Message::At {
                    qq: user_id.to_string(),
                    name: None,
                }, crate::message::Message::Text {
                    text: msg.to_string(),
                }]).await;
        }
        None
    }
    async fn _at_text(&self) -> Option<i64> {
        if let Some(event) = &self.event {
            let user_id = match event {
                MessageEvent::Private(p) => {
                    p.user_id
                }
                MessageEvent::Group(e) => {
                    e.user_id
                }
            };
            return Some(user_id);
        }
        None
    }
    /// 设置临时 Matcher<MessageEvent>
    pub async fn set_temp_message_event_matcher<H>(&self, event: &MessageEvent, handler: H)
        where
            H: Handler<MessageEvent> + Send + Sync + 'static,
    {
        self.set_message_matcher(build_temp_message_event_matcher(None, 30, event, handler))
            .await;
    }
    
    /// 请求消息内容
    ///
    /// 传入 event raw_message 若不为空则直接返回该消息文本（传入 None 表示必须请求）
    ///
    /// 传入 msg 为发送给用户的请求文本信息（传入 None 表示不向用户发送请求信息）
    ///
    /// 重新请求消息为空将返回 None
    pub async fn request_message_callback<F>(
        &self,
        timeout: i64,
        user_id: Option<i64>,
        msg: Option<MessageChain>,
        f: F,
    ) -> NBResult<(String, MessageEvent)>
        where
            F: Fn(String, MessageEvent, Matcher<MessageEvent>) -> std::pin::Pin<Box<dyn Future<Output = bool> + Send>>
    {
        /// 临时 Matcher 的 Handler struct
        struct TempMessageMatcher;
        
        #[async_trait]
        impl Handler<MessageEvent> for TempMessageMatcher {
            // timeout 后调用，通知接受端 Timeout
            fn timeout_drop(&self, matcher: &Matcher<MessageEvent>) {
                let sender = matcher.bot.clone().unwrap().api_sender;
                tokio::spawn(async move { sender.send(ApiChannelItem::TimeOut).await.unwrap() });
            }
            
            fn match_(&mut self, _: &mut MessageEvent) -> bool {
                true
            }
            
            async fn handle(&self, event: MessageEvent, matcher: &mut Matcher<MessageEvent>) {
                matcher
                    .bot
                    .clone()
                    .unwrap()
                    .api_sender
                    .send(ApiChannelItem::MessageEvent(event))
                    .await
                    .unwrap();
            }
        }

        // 搭建临时通道接受 MessageEvent
        let (sender, mut receiver) = tokio::sync::mpsc::channel::<ApiChannelItem>(4);
        let event = self.event.clone().unwrap();
        // 根据提供的 event Handler 构建仅指向当先通话的 Temp Matcher
        let mut m = build_temp_message_event_matcher(user_id, timeout, &event, TempMessageMatcher);
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
        self.set_message_matcher(m).await;

        // Temp Matcher 已就绪，发送提示信息
        if let Some(msg) = msg {
            self.send_(msg).await;
        }
        
        // 等待接收 MessageEvent
        while let Some(data) = receiver.recv().await {
            match data {
                ApiChannelItem::MessageEvent(event) => {
                    if f(remove_space(event.get_raw_message()), event.clone(), self.clone()).await {
                        return Ok((remove_space(event.get_raw_message()), event.clone()));
                    }
                }
                ApiChannelItem::TimeOut => {
                    event!(Level::DEBUG, "Temp Message Matcher TimeOut");
                    return Err(NBError::State(Session::Timeout));
                }
                // 中转 temp Matcher 的 Remove Action
                // ApiChannelItem::Action(action) => self.set(action).await,
                ApiChannelItem::NoticeEvent(_) => {
                    return Err(NBError::State(Session::On));
                }
                ApiChannelItem::RequestEvent(_) => {
                    return Err(NBError::State(Session::On));
                }
                _ => {
                    event!(
                        Level::WARN,
                        "{}",
                        "Temp Message Matcher接受端接收到错误Api或Action消息".bright_red()
                    );
                    return Err(NBError::State(Session::Error));
                } // 忽视 event 该 receiver 永不应该收到 event
            }
        }
        Err(NBError::State(Session::Stop))
    }
    pub async fn request_notice_callback(
        &self,
        timeout: i64,
        user_id: Option<i64>,
        notice_type: Option<NoticeType>,
        sub_type: Option<NoticeSubType>,
        msg: Option<MessageChain>,
    ) -> Option<NoticeEvent> {
        /// 临时 Matcher 的 Handler struct
        struct TempNoticeMatcher;
        
        #[async_trait]
        impl Handler<MessageEvent> for TempNoticeMatcher {
            // timeout 后调用，通知接受端 Timeout
            fn timeout_drop(&self, matcher: &Matcher<MessageEvent>) {
                let sender = matcher.bot.clone().unwrap().api_sender;
                tokio::spawn(async move { sender.send(ApiChannelItem::TimeOut).await.unwrap() });
            }
            
            fn match_(&mut self, _: &mut MessageEvent) -> bool {
                true
            }
            
            async fn handle(&self, event: MessageEvent, matcher: &mut Matcher<MessageEvent>) {
                matcher
                    .bot
                    .clone()
                    .unwrap()
                    .api_sender
                    .send(ApiChannelItem::MessageEvent(event))
                    .await
                    .unwrap();
            }
        }
        
        // 搭建临时通道接受 MessageEvent
        let (sender, mut receiver) = tokio::sync::mpsc::channel::<ApiChannelItem>(4);
        let event = self.event.clone().unwrap();
        // 根据提供的 event Handler 构建仅指向当先通话的 Temp Matcher
        let mut m = build_temp_message_event_matcher(user_id, timeout, &event, TempNoticeMatcher);
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
        self.set_message_matcher(m).await;
        
        // Temp Matcher 已就绪，发送提示信息
        if let Some(msg) = msg {
            self.send_(msg).await;
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
                        "Temp Message Matcher接受端接收到错误Api或Action消息".bright_red()
                    );
                } // 忽视 event 该 receiver 永不应该收到 event
            }
        }
        None
    }
    /// 发送 Vec<Message> 消息 带 message_id
    pub async fn send(&self, msg: crate::message::MessageChain) -> Option<crate::api_resp::MessageId> {
        if let (Some(bot), Some(event)) = (&self.bot, &self.event) {
            bot.send_by_message_event_(&event, msg).await
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
            bot.send_by_message_event(&event, msg).await;
        } else {
            event!(
                Level::ERROR,
                "{}",
                "Sending msg with unbuilt matcher!".red()
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
            let (user_id, group_id) = match event {
                MessageEvent::Private(_) => {
                    return false;
                },
                MessageEvent::Group(g) => {
                    (g.user_id, g.group_id)
                }
            };
            match bot.get_group_member_list(group_id).await {
                None => return false,
                Some(v) => {
                    for x in v {
                        if x.user_id.eq(&user_id) {
                            return match x.role {
                                Role::Owner => true,
                                Role::Admin => true,
                                Role::Member => false,
                            }
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
}
