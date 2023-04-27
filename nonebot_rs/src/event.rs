use crate::message::{Message, MessageChain};
use serde::{Deserialize, Serialize};


/// WebSocket 接受数据枚举 Event || ApiResp
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RecvItem {
    Event(Event),
    ApiResp(crate::api_resp::ApiResp),
}

/// Onebot 事件
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "post_type")]
pub enum Event {
    /// 消息事件
    #[serde(rename = "message")]
    Message(MessageEvent),

    /// 通知事件
    #[serde(rename = "notice")]
    Notice(NoticeEvent),

    /// 请求事件
    #[serde(rename = "request")]
    Request(RequestEvent),

    /// 元事件
    #[serde(rename = "meta_event")]
    Meta(MetaEvent),

    /// Nonebot 内部事件
    #[serde(skip)]
    Nonebot(NbEvent),
}

/// Nonebot Event
#[derive(Debug, Clone)]
pub enum NbEvent {
    BotConnect { bot: crate::Bot },
    BotDisconnect { bot: crate::Bot },
}

/// 消息事件
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "message_type")]
pub enum MessageEvent {
    /// 私聊事件
    #[serde(rename = "private")]
    Private(PrivateMessageEvent),

    /// 群消息事件
    #[serde(rename = "group")]
    Group(GroupMessageEvent),
}

impl MessageEvent {
    /// 消息事件时间戳
    #[allow(dead_code)]
    pub fn get_time(&self) -> i64 {
        match self {
            MessageEvent::Private(p) => p.time,
            MessageEvent::Group(g) => g.time,
        }
    }

    /// 消息事件字符串格式消息
    #[allow(dead_code)]
    pub fn get_raw_message(&self) -> &str {
        match self {
            MessageEvent::Private(p) => &p.raw_message,
            MessageEvent::Group(g) => &g.raw_message,
        }
    }

    /// 消息事件设置字符串格式消息
    #[allow(dead_code)]
    pub fn set_raw_message(&mut self, new_raw_message: String) -> MessageEvent {
        match self {
            MessageEvent::Private(p) => {
                p.raw_message = new_raw_message;
                MessageEvent::Private(p.clone())
            }
            MessageEvent::Group(g) => {
                g.raw_message = new_raw_message;
                MessageEvent::Group(g.clone())
            }
        }
    }

    /// 消息事件数组格式消息
    #[allow(dead_code)]
    pub fn get_message_chain(&self) -> MessageChain {
        match self {
            MessageEvent::Private(p) => p.message.clone(),
            MessageEvent::Group(g) => g.message.clone(),
        }
    }

    /// 消息事件发送者昵称
    #[allow(dead_code)]
    pub fn get_sender_nickname(&self) -> &str {
        match self {
            MessageEvent::Private(p) => &p.sender.nickname,
            MessageEvent::Group(g) => &g.sender.nickname,
        }
    }
    /// 消息id
    #[allow(dead_code)]
    pub fn get_message_id(&self) -> i32 {
        match self {
            MessageEvent::Private(p) => p.message_id,
            MessageEvent::Group(g) => g.message_id,
        }
    }
    /// 消息发送者 user_id
    #[allow(dead_code)]
    pub fn get_sender_user_id(&self) -> i64 {
        match self {
            MessageEvent::Private(p) => p.user_id,
            MessageEvent::Group(g) => g.user_id,
        }
    }
}

/// 私聊消息事件
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrivateMessageEvent {
    /// Event 时间戳
    pub time: i64,
    /// 收到事件的机器人 QQ 号
    pub self_id: i64,
    /// 消息子类型
    pub sub_type: PrivateSubType,
    /// 消息 ID
    pub message_id: i32,
    /// 发送者 ID
    pub user_id: i64,
    /// Array 消息内容
    pub message: MessageChain,
    /// 原生消息内容
    pub raw_message: String,
    /// 字体
    pub font: i32,
    /// 发送者消息
    pub sender: PrivateSender,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PrivateSubType {
    /// 好友
    #[serde(rename = "friend")]
    Friend,
    /// 群聊
    #[serde(rename = "normal")]
    Normal,
    /// 匿名
    #[serde(rename = "anonymous")]
    Anonymous,
    /// 群中自身发送
    #[serde(rename = "group_self")]
    GroupSelf,
    /// 群临时会话
    #[serde(rename = "group	")]
    Group,
    /// 系统提示
    #[serde(rename = "notice")]
    Notice,
}
/// 私聊消息事件发送者
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrivateSender {
    /// 发送者 QQ 号
    pub user_id: i64,
    /// 昵称
    pub nickname: String,
    /// 性别 male|female|unkown
    pub sex: String,
    /// 年龄
    pub age: i32,
}

/// 群消息事件
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupMessageEvent {
    /// Event 时间戳
    pub time: i64,
    /// 收到事件的机器人 QQ 号
    pub self_id: i64,
    /// 消息子类型
    pub sub_type: GroupSubType,
    /// 消息 ID
    pub message_id: i32,
    /// 群消息群号
    pub group_id: i64,
    /// 发送者 ID
    pub user_id: i64,
    /// 匿名消息 非匿名消息为空
    pub anonymous: Option<Anoymous>,
    /// Array 消息内容
    pub message: MessageChain,
    /// 原生消息内容
    pub raw_message: String,
    /// 字体
    pub font: i32,
    /// 发送者消息
    pub sender: GroupSender,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum GroupSubType {
    /// 群聊
    #[serde(rename = "normal")]
    Normal,
    /// 匿名
    #[serde(rename = "anonymous")]
    Anonymous,
    /// 系统提示
    #[serde(rename = "notice")]
    Notice,
}
/// 群消息事件发送者
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupSender {
    /// 发送者 QQ 号
    pub user_id: i64,
    /// 昵称
    pub nickname: String,
    /// 群名片|备注
    pub card: String,
    /// 性别 male|female|unkown
    pub sex: Sex,
    /// 年龄
    pub age: i32,
    /// 地区
    pub area: String,
    /// 成员等级
    pub level: String,
    /// 角色 owner|admin|member
    pub role: Role,
    /// 专属头衔
    pub title: String,
}

/// 角色
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Role {
    /// 群主
    #[serde(rename = "owner")]
    Owner,
    /// 管理员
    #[serde(rename = "admin")]
    Admin,
    /// 群成员
    #[serde(rename = "member")]
    Member,
}

/// 性别
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Sex {
    /// 男
    #[serde(rename = "male")]
    Male,
    /// 女
    #[serde(rename = "female")]
    Female,
    /// 未知
    #[serde(rename = "unknown")]
    Unknown,
}

/// 消息事件匿名字段
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Anoymous {
    /// 匿名用户 ID
    pub id: i64,
    /// 匿名用户名称
    pub name: String,
    /// 匿名用户 flag
    pub flag: String,
}

/// 通知事件
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NoticeEvent {
    /// Event 时间戳
    pub time: i64,
    /// 收到事件的机器人 QQ 号
    pub self_id: i64,
    /// 上报类型
    pub notice_type: NoticeType,
    /// 事件子类型
    pub sub_type: Option<NoticeSubType>,
    /// 群号
    #[serde(default)]
    pub group_id: Option<i64>,
    /// 操作者 QQ 号
    #[serde(default)]
    pub operator_id: Option<i64>,
    /// 发送者 ID
    #[serde(default)]
    pub user_id: i64,
    /// 文件信息
    pub file: Option<File>,
    /// 禁言时长，单位秒
    pub duration: Option<i64>,
    /// 被撤回的消息 ID
    pub message_id: Option<i64>,
    /// 目标 QQ 号
    pub target_id: Option<i64>,
    /// 荣誉类型 talkative:龙王|performer:群聊之火|emotion:快乐源泉
    pub honor_type: Option<HonorType>,

    #[serde(default)]
    pub client: Option<BotClient>,

    #[serde(default)]
    pub online: Option<bool>,

}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BotClient {
    pub app_id: i64,
    pub device_kind: String,
    pub device_name: String,
}
/// 通知类型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum NoticeType {
    /// 群文件上传
    #[serde(rename = "group_upload")]
    GroupUpload,
    /// 群管理员变更
    #[serde(rename = "group_admin")]
    GroupAdmin,
    /// 群成员减少
    #[serde(rename = "group_decrease")]
    GroupDecrease,
    /// 群成员增加
    #[serde(rename = "group_increase")]
    GroupIncrease,
    /// 群成员禁言
    #[serde(rename = "group_ban")]
    GroupBan,
    /// 好友添加
    #[serde(rename = "friend_add")]
    FriendAdd,
    /// 群消息撤回
    #[serde(rename = "group_recall")]
    GroupRecall,
    /// 好友消息撤回
    #[serde(rename = "friend_recall")]
    FriendRecall,
    /// 群名片变更
    #[serde(rename = "group_card")]
    GroupCard,
    /// 离线文件上传
    #[serde(rename = "offline_file")]
    OfflineFile,
    /// 客户端状态变更
    #[serde(rename = "client_status")]
    ClientStatus,
    /// 精华消息
    #[serde(rename = "essence")]
    Essence,
    /// 系统通知
    #[serde(rename = "notify")]
    Notify,

}

/// 通知子类型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum NoticeSubType {
    /// 群荣誉变更
    #[serde(rename = "honor")]
    Honor,
    /// 戳一戳
    #[serde(rename = "poke")]
    Poke,
    /// 群红包幸运王
    #[serde(rename = "lucky_king")]
    LuckyKing,
    /// 群成员头衔变更
    #[serde(rename = "title")]
    Title,
    /// 管理员已同意入群
    #[serde(rename = "approve")]
    Approve,
    /// 管理员邀请入群
    #[serde(rename = "invite")]
    Invite,
    /// 管理员邀请入群
    #[serde(rename = "leave")]
    Leave,
    /// 管理员邀请入群
    #[serde(rename = "kick")]
    Kick,
    /// 管理员邀请入群
    #[serde(rename = "kick_me")]
    KickMe,
    /// 设置和取消管理员
    #[serde(rename = "set")]
    Set,
    #[serde(rename = "unset")]
    Unset,
    /// 表示禁言、解除禁言
    #[serde(rename = "ban")]
    Ban,
    #[serde(rename = "lift_ban")]
    LiftBan,
    /// 精华消息变更
    #[serde(rename = "add")]
    EssenceAdd,
    #[serde(rename = "delete")]
    EssenceDelete,

}

/// 通知子类型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum HonorType {
    /// 龙王
    #[serde(rename = "talkative")]
    Talkative,
    /// 群聊之火
    #[serde(rename = "performer")]
    Performer,
    /// 快乐源泉
    #[serde(rename = "emotion")]
    Emotion,

}

/// 通知事件文件字段
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct File {
    /// 文件 ID
    pub id: String,
    /// 文件名
    pub name: String,
    /// 文件大小（字节数）
    pub size: i64,
    /// 用途未知
    pub busid: i64,
}

/// 请求事件
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestEvent {
    /// Event 时间戳
    pub time: i64,
    /// 收到事件的机器人 QQ 号
    pub self_id: i64,
    /// 请求类型
    pub request_type: RequestType,
    /// 发送请求的 QQ 号
    pub user_id: i64,
    /// 验证信息
    pub comment: String,
    /// 请求 flag
    pub flag: String,
    /// 请求子类型
    pub sub_type: Option<RequestSubType>,
    /// 群号
    #[serde(default)]
    pub group_id: Option<i64>,
}

/// 请求类型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RequestType {
    /// 好友请求
    #[serde(rename = "friend")]
    Friend,
    /// 群请求
    #[serde(rename = "group")]
    Group,
}

/// 请求子类型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RequestSubType {
    /// 加群请求
    #[serde(rename = "add")]
    Add,
    /// 邀请登录号入群
    #[serde(rename = "invite")]
    Invite,
}

/// 元事件
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MetaEvent {
    /// Event 时间戳
    pub time: i64,
    /// 收到事件的机器人 QQ 号
    pub self_id: i64,
    /// 元事件类型 lifecycle|heartbeat
    pub meta_event_type: String,
    /// 事件子类型
    pub sub_type: Option<String>,
    /// 状态信息
    pub status: Option<Status>,
    /// 下次心跳间隔，单位毫秒
    pub interval: Option<i64>,
}

#[test]
fn de_test() {
    let test_str = "{\"group_id\":101,\"message_id\":111,\"notice_type\":\"group_recall\",\"operator_id\":11,\"post_type\":\"notice\",\"self_id\":11,\"time\":1631193409,\"user_id\":11}\n";
    let _meta: Event = serde_json::from_str(test_str).unwrap();
}

/// 元事件状态字段
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Status {
    /// 是否在线，None 表示无法查询
    pub online: Option<bool>,
    /// 运行状态是否符合预期
    pub good: bool,
}

/// `get_user_id()` trait
pub trait UserId {
    fn get_user_id(&self) -> i64;
}

impl UserId for MessageEvent {
    fn get_user_id(&self) -> i64 {
        match self {
            MessageEvent::Private(p) => p.user_id,
            MessageEvent::Group(g) => g.user_id,
        }
    }
}

impl UserId for NoticeEvent {
    fn get_user_id(&self) -> i64 {
        self.user_id
    }
}

impl UserId for RequestEvent {
    fn get_user_id(&self) -> i64 {
        self.user_id
    }
}

/// `get_self_id()` trait
pub trait SelfId {
    fn get_self_id(&self) -> i64;
}

impl SelfId for MessageEvent {
    fn get_self_id(&self) -> i64 {
        match self {
            MessageEvent::Private(p) => p.self_id,
            MessageEvent::Group(g) => g.self_id,
        }
    }
}

impl SelfId for RequestEvent {
    fn get_self_id(&self) -> i64 {
        self.self_id
    }
}

impl SelfId for NoticeEvent {
    fn get_self_id(&self) -> i64 {
        self.self_id
    }
}

impl SelfId for MetaEvent {
    fn get_self_id(&self) -> i64 {
        self.self_id
    }
}

impl SelfId for Event {
    fn get_self_id(&self) -> i64 {
        match self {
            Event::Message(e) => e.get_self_id(),
            Event::Request(e) => e.get_self_id(),
            Event::Notice(e) => e.get_self_id(),
            Event::Meta(e) => e.get_self_id(),
            Event::Nonebot(e) => match e {
                NbEvent::BotConnect { bot } => bot.bot_id,
                NbEvent::BotDisconnect { bot } => bot.bot_id,
            },
        }
    }
}
