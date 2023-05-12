use std::process::id;
use crate::config::BotConfig;
use crate::event::{Event, GroupId, MessageEvent};
use crate::event::{SelfId, UserId};
use crate::matcher::Rule;
use std::sync::Arc;

/// 判定 sender 是否为 superuser
pub fn is_superuser<E>() -> Rule<E>
where
    E: UserId,
{
    let is_superuser = |event: &E, config: &BotConfig| -> bool {
        let user_id = event.get_user_id();
        for superuser in &config.superusers {
            if user_id == superuser.parse::<i64>().expect("内容仅能是整数!") {
                return true;
            }
        }
        false
    };
    Arc::new(is_superuser)
}

/// 判定是否为指定 Bot
pub fn is_bot<E>(bot_id: i64) -> Rule<E>
    where
        E: SelfId,
{
    let is_bot = move |event: &E, _: &BotConfig| -> bool {
        let self_id = event.get_self_id();
        if bot_id == self_id {
            return true;
        }
        false
    };
    Arc::new(is_bot)
}

/// 判定 sender 是否为指定 user
pub fn is_user<E>(user_id: i64) -> Rule<E>
    where
        E: UserId,
{
    let is_user = move |event: &E, _: &BotConfig| -> bool {
        let id = event.get_user_id();
        if id == user_id {
            return true;
        }
        false
    };
    Arc::new(is_user)
}

/// 判定 event 是否来自指定 group
pub fn in_group<E>(group_id: i64) -> Rule<E>
    where
        E: GroupId,
{
    let in_group = move |event: &E, _: &BotConfig| -> bool {
        let id = event.get_group_id();
        if id == group_id {
            return true;
        }
        false
    };
    Arc::new(in_group)
}

/// 判定 event 是否为私聊消息事件
pub fn is_private_message_event() -> Rule<MessageEvent> {
    let is_private_message_event = |event: &MessageEvent, _: &BotConfig| -> bool {
        match event {
            MessageEvent::Group(_) => false,
            MessageEvent::Private(_) => true,
        }
    };
    Arc::new(is_private_message_event)
}

/// 会话开始指令
pub fn on_command(msg: String) -> Rule<MessageEvent> {
    let on_command = move |event: &MessageEvent, _: &BotConfig| -> bool {
        if let MessageEvent::Group(g) = event {
            if g.raw_message.eq(&msg) {
                return true;
            }
        }
        false
    };
    Arc::new(on_command)
}
