use crate::config::BotConfig;
use crate::event::{GroupMessageEvent, MessageEvent, PrivateMessageEvent, SelfId};
use crate::utils::timestamp;
use crate::{Action, Message};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::warn;
use crate::cq_code::*;
use crate::message::MessageChain;

mod action;
#[doc(hidden)]
pub mod api;
#[doc(hidden)]
pub mod matchers;
#[doc(hidden)]
pub mod message_event_matcher;

#[doc(hidden)]
pub mod set_get;
/// 内建 rules
#[cfg(feature = "matcher")]
#[cfg_attr(docsrs, doc(cfg(feature = "matcher")))]
pub mod rules;
#[cfg(feature = "matcher")]
#[cfg_attr(docsrs, doc(cfg(feature = "matcher")))]
pub mod prematchers;

#[macro_export]
macro_rules! matcher {
    ($e:ident,$b:block) => {
        pub fn matcher() -> ::nonebot_rs::prelude::Matcher<$e>{
            $b
        }
    };
}
#[macro_export]
macro_rules! matcher_vec {
    ($e:ident,$b:block) => {
        pub fn matcher() -> Vec<::nonebot_rs::prelude::Matcher<$e>>{
            $b
        }
    };
}

/// rule 函数类型
pub type Rule<E> = Arc<dyn Fn(&E, &BotConfig) -> bool + Send + Sync>;
/// permatcher 函数类型
pub type PreMatcher<E> = fn(&mut E, BotConfig) -> bool;

/// 单个匹配器，参与匹配的最小单元
///
/// Matcher 匹配器，每个匹配器对应一个 handle 函数
#[derive(Clone)]
pub struct Matcher<E>
where
    E: Clone,
{
    /// Matcher 名称，是 Matcher 的唯一性标识
    pub name: String,
    /// Bot
    pub bot: Option<crate::bot::Bot>,
    /// Matchers Action Sender
    action_sender: Option<matchers::ActionSender>,
    /// Matcher 的匹配优先级
    pub priority: i8,
    /// 前处理函数组，获取 &mut event
    pre_matchers: Vec<Arc<PreMatcher<E>>>,
    /// rule 组
    rules: Vec<Rule<E>>,
    /// 是否阻止事件向下一级传递
    pub block: bool,
    /// Matcher 接口函数与可配置项结构体
    handler: Arc<RwLock<dyn Handler<E> + Sync + Send>>,
    /// 是否被禁用
    pub disable: bool,
    /// 是否为临时 Matcher
    pub temp: bool,
    /// 过期时间戳
    pub timeout: Option<i64>,

    #[doc(hidden)]
    event: Option<E>,
}

#[doc(hidden)]
impl<E> std::fmt::Debug for Matcher<E>
where
    E: Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Matcher")
            .field("name", &self.name)
            .field("priority", &self.priority)
            .field("block", &self.block)
            .field("disable", &self.disable)
            .field("temp", &self.temp)
            .field("timeout", &self.timeout)
            .field("bot", &self.bot)
            .finish()
    }
}

/// Matcher 接口 trait
#[async_trait]
pub trait Handler<E>
where
    E: Clone,
{
    /// 新 Bot 连接时，调用该函数
    fn on_bot_connect(&self, _: Matcher<E>) {}
    /// Bot 断开连接时，调用该函数
    fn on_bot_disconnect(&self, _: Matcher<E>) {}
    /// timeout drop 函数
    fn timeout_drop(&self, _: &Matcher<E>) {}
    /// 匹配函数
    fn match_(&mut self, event: &mut E) -> bool;
    /// 处理函数
    async fn handle(&self, event: E, matcher: Matcher<E>);
    /// Load config
    #[allow(unused_variables)]
    fn load_config(&mut self, config: HashMap<String, toml::Value>) {}
}

impl<E> Matcher<E>
where
    E: Clone,
{
    /// 生成默认模板 Matcher
    ///
    /// 默认模板：
    /// ``` rust
    /// Matcher {
    ///     name: name,
    ///     bot: None,
    ///     priority: 1,
    ///     pre_matchers: vec![],
    ///     rules: vec![],
    ///     block: true,
    ///     handler: Arc::new(RwLock::new(handler)),
    ///     disable: false,
    ///     temp: false,
    ///     timeout: None,
    ///     event: None,
    /// }
    /// ```
    pub fn new<H>(name: &str, handler: H) -> Matcher<E>
    where
        H: Handler<E> + Sync + Send + 'static,
    {
        // 默认 Matcher
        Matcher {
            name: name.to_string(),
            bot: None,
            action_sender: None,
            priority: 1,
            pre_matchers: vec![],
            rules: vec![],
            block: true,
            handler: Arc::new(RwLock::new(handler)),
            disable: false,
            temp: false,
            timeout: None,

            event: None,
        }
    }

    #[doc(hidden)]
    fn pre_matcher_handle(&self, event: &mut E, config: BotConfig) -> bool {
        // 遍历 pre_matcher 处理
        for premather in &self.pre_matchers {
            if !premather(event, config.clone()) {
                return false;
            }
        }
        true
    }

    #[doc(hidden)]
    fn check_rules(&self, event: &E, config: &BotConfig) -> bool {
        // 一次性检查当前事件是否满足所有 Rule
        // check the event fit all the rules or not
        for rule in &self.rules {
            if !rule(event, config) {
                return false;
            }
        }
        true
    }

    #[doc(hidden)]
    pub async fn match_(
        &self,
        event: E,
        config: BotConfig,
        matchers: &mut matchers::Matchers,
    ) -> bool
    where
        E: Send + 'static + SelfId,
    {
        // Matcher 处理流程，匹配成功返回 true 并行处理 handler
        let mut event = event.clone();
        if let Some(timeout) = self.timeout {
            if timestamp() > timeout {
                matchers.remove_matcher(&self.name);
                {
                    let handler = self.handler.read().await;
                    handler.timeout_drop(&self);
                }
                return false;
            }
        }
        if self.disable {
            return false;
        }

        if !self.pre_matcher_handle(&mut event, config.clone()) {
            return false;
        }
        if !self.check_rules(&event, &config) {
            return false;
        }
        {
            let mut handler = self.handler.write().await;
            if !handler.match_(&mut event) {
                return false;
            }

            let matcher = self.clone().set_event(&event);
            let handler = self.handler.clone();
            tokio::spawn(async move {
                let handler = handler.read().await;
                handler.handle(event, matcher).await
            });
        }
        return true;
    }

    /// 发送 nbrs 内部设置 Action
    pub async fn set(&self, set: Action) {
        if let Some(bot) = &self.bot {
            bot.action_sender.send(set).await.unwrap();
        }
    }

    /// 向 Matchers 添加 Matcher<MessageEvent>
    pub async fn set_message_matcher(&self, matcher: Matcher<MessageEvent>) {
        let action = action::MatchersAction::AddMessageEventMatcher {
            message_event_matcher: matcher,
        };
        if let Some(action_sender) = &self.action_sender {
            action_sender.send(action).unwrap();
        } else {
            tracing::event!(tracing::Level::WARN, "Action Sender not init.")
        }
    }
}

/// 构建 timeout 为 30s 的临时 Matcher<MessageEvent>
pub fn build_temp_message_event_matcher<H>(
    event: &MessageEvent,
    handler: H,
) -> Matcher<MessageEvent>
where
    H: Handler<MessageEvent> + Send + Sync + 'static,
{
    use crate::event::UserId;
    let mut m = Matcher::new(
        &format!(
            "{}-{}-{}",
            event.get_self_id(),
            event.get_user_id(),
            event.get_time()
        ),
        handler,
    )
        .add_rule(crate::matcher::rules::is_user(event.get_user_id()))
        .add_rule(crate::matcher::rules::is_bot(event.get_self_id()));
    if let MessageEvent::Group(g) = event {
        m.add_rule(crate::matcher::rules::in_group(g.group_id));
    } else {
        m.add_rule(crate::matcher::rules::is_private_message_event());
    }
    m.set_priority(0)
        .set_temp(true)
        .set_timeout(timestamp() + 30)
}

#[derive(Clone, Debug)]
pub enum EventArg {
    All(Vec<EventArg>),
    Not(Vec<EventArg>),
    Regexp(String),
}

#[derive(Clone)]
pub enum HandEvent<'a> {
    MessageEvent(&'a MessageEvent, String),
    PrivateMessageEvent(&'a PrivateMessageEvent, String),
    GroupMessageEvent(&'a GroupMessageEvent, String),
}

impl HandEvent<'_> {
    pub fn content(&self) -> &'_ String {
        match self {
            HandEvent::MessageEvent(_, content) => &content,
            HandEvent::PrivateMessageEvent(_, content) => &content,
            HandEvent::GroupMessageEvent(_, content) => &content,
        }
    }
}

impl<'a> From<&'a mut MessageEvent> for HandEvent<'a> {
    fn from(value: &'a mut MessageEvent) -> Self {
        Self::MessageEvent(value, value.get_raw_message().to_string())
    }
}

impl<'a> From<&'a mut PrivateMessageEvent> for HandEvent<'a> {
    fn from(value: &'a mut PrivateMessageEvent) -> Self {
        Self::PrivateMessageEvent(value, value.raw_message.clone())
    }
}

impl<'a> From<&'a mut GroupMessageEvent> for HandEvent<'a> {
    fn from(value: &'a mut GroupMessageEvent) -> Self {
        Self::GroupMessageEvent(value, value.raw_message.clone())
    }
}

pub fn match_event_args_all(args: Vec<EventArg>, event: HandEvent) -> bool {
    for x in args {
        if !match_event_item(x, event.clone()) {
            return false;
        }
    }
    // 一个条件都没有认为是true
    true
}

fn match_event_args_not(args: Vec<EventArg>, event: HandEvent) -> bool {
    for x in args {
        if match_event_item(x, event.clone()) {
            return false;
        }
    }
    true
}

fn match_event_args_regexp(args: String, event: HandEvent) -> bool {
    regex::Regex::new(args.as_str()).expect("正则表达式构建错误").is_match(event.content().as_str())
}

fn match_event_item(arg: EventArg, event: HandEvent) -> bool {
    match arg {
        EventArg::All(v) => match_event_args_all(v, event.clone()),
        EventArg::Not(v) => match_event_args_not(v, event.clone()),
        EventArg::Regexp(v) => match_event_args_regexp(v, event.clone()),
    }
}

pub struct CommandMatcher {
    pub idx: usize,
    pub elements: MessageChain,
    pub matching: String,
}

impl CommandMatcher {
    pub fn new(value: MessageChain) -> CommandMatcher {
        let mut matcher = CommandMatcher {
            idx: 0,
            elements: value,
            matching: String::new(),
        };
        matcher.push_text();
        matcher
    }

    pub fn push_text(&mut self) {
        loop {
            if self.idx >= self.elements.len() {
                break;
            }
            let ele: &Message = self.elements.get(self.idx).unwrap();
            match ele {
                Message::Text { text } => {
                    self.matching.push_str(text.as_str());
                    self.idx += 1;
                }
                _ => break,
            }
        }
        let build = self.matching.trim().to_string();
        self.matching = build;
    }

    pub fn match_command(&mut self, command_name: &str) -> bool {
        let sp_regexp = regex::Regex::new("\\s+").expect("nonebot_rs 正则错误");
        let mut sp = sp_regexp.split(self.matching.as_str());
        if let Some(first) = sp.next() {
            if command_name.eq(first) {
                self.matching = self.matching[first.len()..].trim().to_string();
                return true;
            }
        }
        return false;
    }

    pub fn not_blank(&self) -> bool {
        !self.matching.is_empty() || self.idx < self.elements.len()
    }

    pub fn tuple_matcher(&mut self, elements: Vec<TupleMatcherElement>) -> Option<Vec<String>> {
        if self.matching.is_empty() {
            None
        } else {
            warn!("{:?}", elements);
            // matching 恒不为空，至少有1节
            let mut saw = self.matching.split_ascii_whitespace();
            let first = saw.next().unwrap();
            let mut params_match: Vec<&str> = Vec::new();
            let mut params_holding = false;
            let mut sub_match = first;
            for ele in elements {
                match ele {
                    TupleMatcherElement::Command(data) => {
                        if params_holding {
                            if let Some(find) = sub_match.find(data) {
                                params_match.push(&sub_match[..find]);
                                sub_match = &sub_match[find..];
                                sub_match = &sub_match[data.len()..];
                                params_holding = false;
                            } else {
                                return None;
                            }
                        } else {
                            // 第一次匹配
                            if sub_match.starts_with(data) {
                                sub_match = &sub_match[data.len()..];
                            } else {
                                return None;
                            }
                        }
                    }
                    TupleMatcherElement::Param => {
                        if params_holding {
                            return None;
                        } else {
                            params_holding = true;
                        }
                    }
                }
            }
            // 最后一个参数
            if params_holding {
                params_match.push(&sub_match);
            }
            let result = params_match.iter().map(|s| s.to_string()).collect();
            self.matching = self.matching[first.len()..].trim().to_string();
            warn!("{:?}", result);
            Some(result)
        }
    }
}

pub trait FromCommandMatcher: Sized {
    fn get(s: &mut CommandMatcher) -> Option<Self>;
}

#[inline]
pub fn matcher_get<F: Sized + FromCommandMatcher>(matcher: &mut CommandMatcher) -> Option<F> {
    F::get(matcher)
}

impl FromCommandMatcher for String {
    fn get(matcher: &mut CommandMatcher) -> Option<Self> {
        if matcher.matching.is_empty() {
            return None;
        }
        let sp_regexp = regex::Regex::new("\\s+").expect("nonebot_rs 正则错误");
        let mut sp = sp_regexp.split(matcher.matching.as_str());
        if let Some(first) = sp.next() {
            let result = Some(first.to_string());
            matcher.matching = matcher.matching[first.len()..].trim().to_string();
            return result;
        }
        None
    }
}

impl FromCommandMatcher for Option<String> {
    fn get(matcher: &mut CommandMatcher) -> Option<Self> {
        let mut result = None;
        if matcher.matching.is_empty() {
            return Some(result);
        }
        let sp_regexp = regex::Regex::new("\\s+").expect("nonebot_rs 正则错误");
        let mut sp = sp_regexp.split(matcher.matching.as_str());
        if let Some(first) = sp.next() {
            result = Some(first.to_string());
            matcher.matching = matcher.matching[first.len()..].trim().to_string();
        }
        Some(result)
    }
}

impl FromCommandMatcher for Vec<String> {
    fn get(matcher: &mut CommandMatcher) -> Option<Self> {
        let sp_regexp = regex::Regex::new("\\s+").expect("nonebot_rs 正则错误");
        let result = sp_regexp
            .split(matcher.matching.as_str())
            .filter_map(|s| {
                if !s.is_empty() {
                    Some(s.to_string())
                } else {
                    None
                }
            })
            .collect();
        matcher.matching = String::default();
        Some(result)
    }
}

macro_rules! command_base_ty_supplier {
    ($ty:ty) => {
        impl FromCommandMatcher for $ty {
            fn get(matcher: &mut CommandMatcher) -> Option<$ty> {
                if matcher.matching.is_empty() {
                    return None;
                }
                let sp_regexp = regex::Regex::new("\\s+").expect("nonebot_rs 正则错误");
                let mut sp = sp_regexp.split(matcher.matching.as_str());
                if let Some(first) = sp.next() {
                    let result = match first.parse::<$ty>() {
                        Ok(value) => Some(value),
                        Err(_) => return None,
                    };
                    matcher.matching = matcher.matching[first.len()..].trim().to_string();
                    return result;
                }
                None
            }
        }

        impl FromCommandMatcher for Option<$ty> {
            fn get(matcher: &mut CommandMatcher) -> Option<Self> {
                let mut result = None;
                if matcher.matching.is_empty() {
                    return Some(result);
                }
                let sp_regexp = regex::Regex::new("\\s+").expect("nonebot_rs 正则错误");
                let mut sp = sp_regexp.split(matcher.matching.as_str());
                if let Some(first) = sp.next() {
                    match first.parse::<$ty>() {
                        Ok(value) => {
                            result = Some(value);
                            matcher.matching = matcher.matching[first.len()..].trim().to_string();
                        }
                        _ => {}
                    };
                }
                return Some(result);
            }
        }

        impl FromCommandMatcher for Vec<$ty> {
            fn get(matcher: &mut CommandMatcher) -> Option<Self> {
                let mut result = vec![];
                if matcher.matching.is_empty() {
                    return Some(result);
                }
                let sp_regexp = regex::Regex::new("\\s+").expect("nonebot_rs 正则错误");
                let sp = sp_regexp.split(matcher.matching.as_str());
                let mut new_matching = vec![];
                for x in sp {
                    if !new_matching.is_empty() {
                        new_matching.push(x);
                    } else {
                        match x.parse::<$ty>() {
                            Ok(value) => result.push(value),
                            Err(_) => {
                                if result.is_empty() {
                                    return Some(result);
                                } else {
                                    new_matching.push(x);
                                }
                            }
                        }
                    }
                }
                matcher.matching = new_matching.join(" ");
                Some(result)
            }
        }
    };
}

command_base_ty_supplier!(i8);
command_base_ty_supplier!(u8);
command_base_ty_supplier!(i16);
command_base_ty_supplier!(u16);
command_base_ty_supplier!(i32);
command_base_ty_supplier!(u32);
command_base_ty_supplier!(i64);
command_base_ty_supplier!(u64);
command_base_ty_supplier!(i128);
command_base_ty_supplier!(u128);
command_base_ty_supplier!(isize);
command_base_ty_supplier!(usize);
command_base_ty_supplier!(f32);
command_base_ty_supplier!(f64);
command_base_ty_supplier!(bool);
command_base_ty_supplier!(char);

macro_rules! command_rq_element_ty_supplier {
    ($ty:ty, $mat:path) => {
        impl FromCommandMatcher for $ty {
            fn get(matcher: &mut CommandMatcher) -> Option<Self> {
                if !matcher.matching.is_empty() {
                    return None;
                }
                if matcher.idx >= matcher.elements.len() {
                    return None;
                }
                match message_to_cq(matcher.elements.get(matcher.idx).unwrap().clone()) {
                    $mat(i) => {
                        matcher.idx += 1;
                        matcher.push_text();
                        Some(i)
                    }
                    _ => None,
                }
            }
        }

        impl FromCommandMatcher for Option<$ty> {
            fn get(matcher: &mut CommandMatcher) -> Option<Self> {
                let mut result = None;
                if !matcher.matching.is_empty() {
                    return Some(result);
                }
                if matcher.idx >= matcher.elements.len() {
                    return Some(result);
                }
                match message_to_cq(matcher.elements.get(matcher.idx).unwrap().clone()) {
                    $mat(i) => {
                        result = Some(i);
                        matcher.idx += 1;
                        matcher.push_text();
                    }
                    _ => (),
                }
                Some(result)
            }
        }

        impl FromCommandMatcher for Vec<$ty> {
            fn get(matcher: &mut CommandMatcher) -> Option<Self> {
                let mut result = vec![];
                if !matcher.matching.is_empty() {
                    return Some(result);
                }
                loop {
                    if matcher.idx >= matcher.elements.len() {
                        break;
                    }
                    match message_to_cq(matcher.elements.get(matcher.idx).unwrap().clone()) {
                        $mat(i) => {
                            result.push(i);
                            matcher.idx += 1;
                            matcher.push_text();
                        }
                        _ => break,
                    }
                }
                Some(result)
            }
        }
    };
}

command_rq_element_ty_supplier!(At, CqCode::At);
command_rq_element_ty_supplier!(Face, CqCode::Face);
command_rq_element_ty_supplier!(Reply, CqCode::Reply);
command_rq_element_ty_supplier!(Share, CqCode::Share);
command_rq_element_ty_supplier!(Video, CqCode::Video);
command_rq_element_ty_supplier!(Record, CqCode::Record);
command_rq_element_ty_supplier!(Music, CqCode::Music);
command_rq_element_ty_supplier!(Poke, CqCode::Poke);
command_rq_element_ty_supplier!(Forward, CqCode::Forward);
command_rq_element_ty_supplier!(Node, CqCode::Node);
command_rq_element_ty_supplier!(XmlMsg, CqCode::XmlMsg);
command_rq_element_ty_supplier!(JsonMsg, CqCode::JsonMsg);

impl FromCommandMatcher for Image {
    fn get(matcher: &mut CommandMatcher) -> Option<Self> {
        if !matcher.matching.is_empty() {
            return None;
        }
        if matcher.idx >= matcher.elements.len() {
            return None;
        }
        match message_to_cq(matcher.elements.get(matcher.idx).unwrap().clone()) {
            CqCode::Image(i) => {
                matcher.idx += 1;
                matcher.push_text();
                Some(i)
            }
            _ => None,
        }
    }
}

impl FromCommandMatcher for Option<Image> {
    fn get(matcher: &mut CommandMatcher) -> Option<Self> {
        let mut result = None;
        if !matcher.matching.is_empty() {
            return Some(result);
        }
        if matcher.idx >= matcher.elements.len() {
            return Some(result);
        }
        match message_to_cq(matcher.elements.get(matcher.idx).unwrap().clone()) {
            CqCode::Image(i) => {
                result = Some(i);
                matcher.idx += 1;
                matcher.push_text();
            }
            _ => (),
        }
        Some(result)
    }
}

impl FromCommandMatcher for Vec<Image> {
    fn get(matcher: &mut CommandMatcher) -> Option<Self> {
        let mut result = vec![];
        if !matcher.matching.is_empty() {
            return Some(result);
        }
        loop {
            if matcher.idx >= matcher.elements.len() {
                break;
            }
            match message_to_cq(matcher.elements.get(matcher.idx).unwrap().clone()) {
                CqCode::Image(i) => {
                    result.push(i);
                    matcher.idx += 1;
                    matcher.push_text();
                }
                _ => break,
            }
        }
        Some(result)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TupleMatcherElement {
    Command(&'static str),
    Param,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TupleMatcher(String);

impl TupleMatcher {
    pub fn new(context: String) -> Self {
        Self(context)
    }
}

pub trait FromTupleMatcher: Sized {
    fn get(matcher: TupleMatcher) -> Option<Self>;
}

#[inline]
pub fn tuple_matcher_get<F: Sized + FromTupleMatcher>(matcher: TupleMatcher) -> Option<F> {
    F::get(matcher)
}

impl FromTupleMatcher for String {
    fn get(matcher: TupleMatcher) -> Option<Self> {
        if matcher.0.is_empty() {
            None
        } else {
            Some(matcher.0)
        }
    }
}

impl FromTupleMatcher for Option<String> {
    fn get(matcher: TupleMatcher) -> Option<Self> {
        if matcher.0.is_empty() {
            Some(None)
        } else {
            Some(Some(matcher.0))
        }
    }
}

impl FromTupleMatcher for Vec<String> {
    fn get(matcher: TupleMatcher) -> Option<Self> {
        if matcher.0.is_empty() {
            Some(vec![])
        } else {
            Some(vec![matcher.0])
        }
    }
}

impl FromTupleMatcher for Vec<Option<String>> {
    fn get(matcher: TupleMatcher) -> Option<Self> {
        if matcher.0.is_empty() {
            Some(vec![])
        } else {
            Some(vec![Some(matcher.0)])
        }
    }
}

macro_rules! tuple_base_ty_supplier {
    ($ty:ty) => {
        impl FromTupleMatcher for $ty {
            fn get(matcher: TupleMatcher) -> Option<Self> {
                matcher.0.parse::<$ty>().ok()
            }
        }

        impl FromTupleMatcher for Option<$ty> {
            fn get(matcher: TupleMatcher) -> Option<Self> {
                if matcher.0.is_empty() {
                    Some(None)
                } else {
                    matcher.0.parse::<$ty>().ok().map(|v| Some(v))
                }
            }
        }

        impl FromTupleMatcher for Vec<$ty> {
            fn get(matcher: TupleMatcher) -> Option<Self> {
                if matcher.0.is_empty() {
                    Some(vec![])
                } else {
                    matcher.0.parse::<$ty>().ok().map(|v| vec![v])
                }
            }
        }
    };
}

tuple_base_ty_supplier!(i8);
tuple_base_ty_supplier!(u8);
tuple_base_ty_supplier!(i16);
tuple_base_ty_supplier!(u16);
tuple_base_ty_supplier!(i32);
tuple_base_ty_supplier!(u32);
tuple_base_ty_supplier!(i64);
tuple_base_ty_supplier!(u64);
tuple_base_ty_supplier!(i128);
tuple_base_ty_supplier!(u128);
tuple_base_ty_supplier!(isize);
tuple_base_ty_supplier!(usize);
tuple_base_ty_supplier!(f32);
tuple_base_ty_supplier!(f64);
tuple_base_ty_supplier!(bool);
tuple_base_ty_supplier!(char);

fn message_to_cq(m: Message) -> CqCode {
    match m {
        Message::Text { text } => {
            CqCode::Text(text)
        }
        Message::Face { id } => {
            CqCode::Face(Face {
                id,
            })
        }
        Message::Image { file, ty, url, cache, .. } => {
            CqCode::Image(Image {
                file,
                ty,
                url,
                cache,
                ..Default::default()
            })
        }
        Message::Record { file, magic, url, cache, proxy, timeout } => {
            CqCode::Record(Record {
                file,
                magic,
                url,
                cache,
                proxy,
                timeout,
            })
        }
        Message::Video { file, cover, c } => {
            CqCode::Video(Video {
                file,
                cover,
                c,
            })
        }
        Message::At { qq, name } => {
            CqCode::At(At { qq, name })
        }
        Message::Rps => {
            CqCode::Rps
        }
        Message::Dice => {
            CqCode::Dice
        }
        Message::Shake => {
            CqCode::Shake
        }
        Message::Poke { qq } => {
            CqCode::Poke(Poke { qq })
        }
        Message::Anonymous => {
            CqCode::Anonymous
        }
        Message::Share { url, title, content, image } => {
            CqCode::Share(Share { url, title, content, image })
        }
        Message::Contact { ty, id } => {
            CqCode::Contact(Contact { ty, id })
        }
        Message::Lacation { lat, lon, title, content } => {
            CqCode::Location(Location { lat, lon, title, content })
        }
        Message::Music { ty, id, url, audio, title, content, image } => {
            CqCode::Music(Music { ty, id: id.unwrap_or_default(), audio, title, content, image })
        }
        Message::Reply { id, text, qq, time, seq } => {
            CqCode::Reply(Reply { id, text, qq, time, seq })
        }
        Message::Forward { id } => {
            CqCode::Forward(Forward { id })
        }
        Message::Node { id, uin, name, content, seq } => {
            CqCode::Node(Node {
                id,
                name,
                uin,
                content,
                seq,
            })
        }
        Message::Xml { data } => {
            CqCode::XmlMsg(XmlMsg { data })
        }
        Message::Json { data } => {
            CqCode::JsonMsg(JsonMsg { data })
        }
    }
}
