use crate::bot::ApiSender;
use crate::event::{Events, MessageEvent};
use crate::results::HandlerResult;
use crate::Nonebot;
use async_trait::async_trait;
use std::sync::{Arc, Mutex};

pub type AMNb = Arc<Mutex<Nonebot>>;
pub type Rule = fn(&Events, AMNb) -> bool;
pub type MatchersVec<T> = Vec<Box<Matcher<dyn Handler<T>>>>;

#[derive(Clone)]
pub struct Matcher<T> {
    // Matcher 匹配器，每个匹配器对应一个 handle 函数
    pub rules: Vec<Rule>,           // 所有需要被满足的 rule
    pub block: bool,                // 是否阻止事件向下一级传递
    pub temp: bool,                 // 是否为临时 Matcher
    pub handler: T,                 // struct impl Handler trait
    pub disable: bool,              // 禁用当前 Matcher
    pub ignore_command_start: bool, // todo
}

#[async_trait]
pub trait Handler<E> {
    async fn handle(self, event: E, amnb: AMNb, sender: ApiSender) -> HandlerResult;
}

impl<T> Matcher<T>
where
    T: 'static + Handler<MessageEvent>,
{
    pub fn get_rules(&self) -> &Vec<Rule> {
        // 获取当前 Matcher 所有匹配规格
        // get all rules in the Matcher
        &self.rules
    }

    pub fn is_block(&self) -> bool {
        self.block
    }

    pub fn is_temp(&self) -> bool {
        self.temp
    }

    pub fn push_rule(&mut self, rule: Rule) -> Result<(), String> {
        // 给当前 Matcher 增加需要满足的 Rule
        // 可以在此处增加 Rule 的合法性检查
        // check the rule pushable here
        self.rules.push(rule);
        Ok(())
    }

    fn check(&self, event: &Events, nb: AMNb) -> bool {
        // 一次性检查当前事件是否满足所有 Rule
        // check the event fit all the rules or not
        for rule in &self.rules {
            if !rule(event, nb.clone()) {
                return false;
            }
        }
        true
    }

    fn command_start(&self, event: &MessageEvent, nb: AMNb) -> Option<MessageEvent> {
        let raw_message: String;
        raw_message = event.get_raw_message().to_string();
        let cs = {
            (nb.lock()
                .unwrap()
                .bots
                .get(&event.get_self_id())
                .unwrap()
                .command_start)
                .clone()
        };
        for sc in &cs {
            if raw_message.starts_with(sc) {
                let new_raw_message = raw_message[sc.len() - 1..].to_string();
                return Some(event.set_raw_message(new_raw_message));
            }
        }
        None
    }

    pub async fn match_(self, event: MessageEvent, nb: AMNb, sender: ApiSender) -> HandlerResult {
        let r = tokio::spawn(self.handler.handle(event.clone(), nb.clone(), sender));
        r.await.unwrap()
    }
}
