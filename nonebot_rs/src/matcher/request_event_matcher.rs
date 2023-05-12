use tracing::{event, Level};
use crate::event::{RequestEvent, RequestType};
use crate::matcher::Matcher;
use colored::Colorize;

impl Matcher<RequestEvent> {
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
            bot.send_by_request_event_(event, msg).await
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
            bot.send_by_request_event(event, msg).await;
        } else {
            event!(
                Level::ERROR,
                "{}",
                "Sending msg with unbuilt matcher!".red()
            );
        }
    }
    pub async fn accept(&self) {
        if let (Some(bot), Some(event)) = (&self.bot, &self.event) {
            match &event.request_type {
                RequestType::Friend => {
                    bot.set_friend_add_request(event.flag.clone(), true, "".to_string()).await;
                }
                RequestType::Group => {
                    bot.set_group_add_request(event.flag.clone(), event.sub_type.clone().unwrap().to_string(), true, "".to_string()).await;
                }
            }
        } else {
            event!(
                Level::ERROR,
                "{}",
                "Failure to accept request ".red()
            );
        }
    }
    pub async fn reject(&self, reason: Option<&str>) {
        if let (Some(bot), Some(event)) = (&self.bot, &self.event) {
            match &event.request_type {
                RequestType::Friend => {
                    bot.set_friend_add_request(event.flag.clone(), false, "".to_string()).await;
                }
                RequestType::Group => {
                    bot.set_group_add_request(event.flag.clone(), event.sub_type.clone().unwrap().to_string(), false, reason.unwrap_or("").to_string()).await;
                }
            }
        } else {
            event!(
                Level::ERROR,
                "{}",
                "Failed to reject request".red()
            );
        }
    }
}