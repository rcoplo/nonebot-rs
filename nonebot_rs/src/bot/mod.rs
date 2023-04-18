use crate::{api_resp, Message};
use crate::event::{MessageEvent, UserId};
use crate::{api, config, utils, ApiChannelItem, ApiResp};
use colored::*;
use tokio::sync::{mpsc, watch};
use tracing::{event, Level};

mod _api;

/// 为 Plugin 提供各类 Onebot Api 
#[derive(Debug, Clone)]
pub struct Bot {
    /// bot id
    pub bot_id: i64,
    /// connect timestamp
    pub connect_time: i64,
    // Bot Config
    pub config: config::BotConfig,
    /// 暂存调用 Bot api
    pub api_sender: mpsc::Sender<ApiChannelItem>,
    /// Nonebot Action Sender
    pub action_sender: crate::ActionSender,
    /// ApiResp Receiver
    pub api_resp_watcher: watch::Receiver<ApiResp>,
}

impl Bot {
    pub fn new(
        bot_id: i64,
        config: config::BotConfig,
        api_sender: mpsc::Sender<ApiChannelItem>,
        action_sender: crate::ActionSender,
        api_resp_watcher: watch::Receiver<ApiResp>,
    ) -> Self {
        Bot {
            bot_id,
            connect_time: crate::utils::timestamp(),
            config,
            api_sender,
            action_sender,
            api_resp_watcher,
        }
    }

    /// Send Group Msg
    pub async fn send_group_msg_nrv(&self, group_id: i64, msg: crate::message::MessageVec) {
        self.api_sender
            .send(ApiChannelItem::Api(crate::api::Api::send_group_msg(
                crate::api::SendGroupMsg {
                    group_id,
                    message: msg.clone(),
                    auto_escape: false,
                },
            )))
            .await
            .unwrap();

        event!(
            Level::INFO,
            "Bot [{}] Send {:.100?} to Group ({})",
            self.config.bot_id.to_string().red(),
            msg,
            group_id.to_string().magenta()
        );
    }

    /// Send Private Msg
    pub async fn send_private_msg_nrv(&self, user_id: i64, msg: crate::message::MessageVec) {
        self.api_sender
            .send(ApiChannelItem::Api(crate::api::Api::send_private_msg(
                crate::api::SendPrivateMsg {
                    user_id,
                    message: msg.clone(),
                    auto_escape: false,
                },
            )))
            .await
            .unwrap();

        event!(
            Level::INFO,
            "Bot [{}] Send {:.100?} to Friend ({})",
            self.config.bot_id.to_string().red(),
            msg,
            user_id.to_string().green()
        );
    }

    /// 根据 MessageEvent 类型发送私聊消息或群消息
    pub async fn send_by_message_event_(&self, event: &MessageEvent, msg: crate::message::MessageVec) -> Option<crate::api_resp::MessageId> {
        match event {
            MessageEvent::Private(p) => self.send_private_msg(p.user_id, msg, false).await,
            MessageEvent::Group(g) => self.send_group_msg(g.group_id, msg, false).await,
        }
    }
    /// 根据 MessageEvent 类型发送私聊消息或群消息 不带返回值
    pub async fn send_by_message_event(&self, event: &MessageEvent, msg: crate::message::MessageVec) {
        match event {
            MessageEvent::Private(p) => self.send_private_msg_nrv(p.user_id, msg).await,
            MessageEvent::Group(g) => self.send_group_msg_nrv(g.group_id, msg).await,
        }
    }

    /// 请求 Onebot Api，不等待 Onebot 返回
    pub async fn call_api(&self, api: api::Api) {
        self.api_sender
            .send(ApiChannelItem::Api(api.clone()))
            .await
            .unwrap();
        event!(
            Level::INFO,
            "Bot [{}] Calling Api {:?}",
            self.config.bot_id.to_string().red(),
            api
        );
    }

    /// 请求 Onebot Api，等待 Onebot 返回项（30s 后 timeout 返回 None）
    pub async fn call_api_resp(&self, api: api::Api) -> Option<api_resp::ApiResp> {
        let echo = api.get_echo();
        self.api_sender
            .send(ApiChannelItem::Api(api.clone()))
            .await
            .unwrap();
        event!(
            Level::INFO,
            "Bot [{}] Calling Api {:?}",
            self.config.bot_id.to_string().red(),
            api
        );
        let time = utils::timestamp();
        let mut watcher = self.api_resp_watcher.clone();
        while let Ok(_) = watcher.changed().await {
            let resp = self.api_resp_watcher.borrow().clone();
            if resp.echo == echo {
                return Some(resp);
            }
            if utils::timestamp() > time + 30 {
                return None;
            }
        }
        None
    }
}
