use crate::builtin;
use crate::event::MessageEvent;
use crate::matcher::{Handler, Matcher};
use crate::on_command;
use async_trait::async_trait;
use rcnb_rs::encode;

#[derive(Clone)]
pub struct Rcnb {}

#[async_trait]
impl Handler<MessageEvent> for Rcnb {
    on_command!(MessageEvent, "rcnb", "RCNB", "Rcnb");
    async fn handle(&self, event: MessageEvent, matcher: Matcher<MessageEvent>) {
        let msg = matcher
            .request_message(Some(&event), Some("Please enter something."))
            .await;
        if let Some(msg) = msg {
            matcher.send_text(&encode(&msg)).await;
        }
    }
}

pub fn rcnb() -> Matcher<MessageEvent> {
    Matcher::new("Rcnb", Rcnb {})
        .add_pre_matcher(builtin::prematcher::to_me())
        .add_pre_matcher(builtin::prematcher::command_start())
}
