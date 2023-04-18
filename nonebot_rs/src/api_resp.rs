use serde::{Deserialize, Serialize};


/// Onebot Api 响应根结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiResp {
    pub status: String,
    pub retcode: i32,
    pub data: RespData,
    pub echo: String,
}

// impl ApiResp {
//     pub fn get_date<D>(&self) -> Option<D> {
//         match self.data {
//             RespData::MessageId(d) => Some(d),
//             _ => None,
//         }
//     }
// }

/// Onebot Api 响应 data 字段
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum RespData {
    None,
    MessageId(MessageId),
    Message(Message),
    Messages(Messages),
    LoginInfo(LoginInfo),
    StrangerInfo(StrangerInfo),
    FriendList(Vec<FriendListItem>),
    GroupInfo(GroupInfo),
    GroupList(Vec<GroupListItem>),
    GroupMemberInfo(GroupMemberInfo),
    GroupMemberList(Vec<GroupMember>),
    GroupHonorInfo(GroupHonorInfo),
    Cookies(Cookies),
    ScrfToken(ScrfToken),
    Credentials(Credentials),
    File(File),
    SendCheck(SendCheck),
    Status(crate::event::Status),
    VersionInfo(VersionInfo),
    ModelVariants(ModelVariants),
    BotOnlineClients(BotOnlineClients),
    UnidirectionalFriendList(Vec<UnidirectionalFriendList>),
    ForwardMsgId(ForwardMsgId),
    GroupMsgHistory(GroupMsgHistory),
    OcrImage(OcrImages),
    GroupSystemMsg(GroupSystemMsg),
    EssenceMsgList(Vec<EssenceMsgList>),
    GroupAtAllRemain(GroupAtAllRemain),
    GroupNotice(GroupNotice),
    GroupFileSystemInfo(GroupFileSystemInfo),
    GroupRootFiles(GroupRootFiles),
    GroupFilesByFolder(GroupFilesByFolder),
    GroupFileUrl(GroupFileUrl),
    DownloadFile(DownloadFiles),
    UrlSafely(UrlSafely)
}

/// message_id 响应数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageId {
    pub message_id: i32,
}

/// get_msg 响应数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub time: i32,
    pub message_type: String,
    pub message_id: i32,
    pub real_id: i32,
    pub sender: Sender,
    pub message: crate::message::MessageVec,
}

/// get_forward_msg 响应数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Messages {
    pub message: crate::message::MessageVec,
}

/// get_login_info 响应数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginInfo {
    pub user_id: i64,
    pub nickname: String,
}

/// get_stranger_info 响应数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StrangerInfo {
    pub user_id: i64,
    pub nickname: String,
    pub sex: crate::event::Sex,
    pub age: i32,
}

/// get_group_info 响应数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupInfo {
    pub group_id: i64,
    pub group_name: String,
    pub member_count: i32,
    pub max_member_count: i32,
}

/// get_group_member_info 响应数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupMemberInfo {
    pub group_id: i64,
    pub user_id: i64,
    pub nickname: String,
    pub card: String,
    pub sex: crate::event::Sex,
    pub age: i32,
    pub area: String,
    pub join_time: i32,
    pub last_sent_time: i32,
    pub level: String,
    pub role: crate::event::Role,
    pub unfriendly: bool,
    pub title: String,
    pub title_expire_time: i32,
    pub card_changeable: bool,
}

/// get_group_honor_info 响应数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupHonorInfo {
    pub group_id: i64,
    pub current_talkative: Option<CurrentTalkative>,
    pub talkative_list: Option<Vec<HonorItem>>,
    pub performer_list: Option<Vec<HonorItem>>,
    pub legend_list: Option<Vec<HonorItem>>,
    pub strong_newbie_list: Option<Vec<HonorItem>>,
    pub emotion_list: Option<Vec<HonorItem>>,
}

/// get_cookies 响应数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cookies {
    pub cookies: String,
}

/// get_csrf_token 响应数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScrfToken {
    pub token: i32,
}

/// get_credentials 响应数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Credentials {
    pub cookies: String,
    pub token: i32,
}

/// get_recode && get_image 响应数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct File {
    pub file: String,
}

/// can_send_image && can_send_record 响应数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendCheck {
    pub yes: bool,
}

/// get_version_info 响应数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VersionInfo {
    pub app_name: String,
    pub app_version: String,
    pub protocol_version: String,
}

/// get_friend_list 响应数组成员
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FriendListItem {
    pub user_id: i64,
    pub nickname: String,
    pub remark: String,
}

/// get_group_list 响应数组成员
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupListItem {
    pub group_id: i64,
    pub group_name: String,
    pub member_count: i32,
    pub max_member_count: i32,
}

/// get_group_member_list 响应数组成员
#[derive(Debug, Serialize, Deserialize, Clone)] // need check
pub struct GroupMember {
    pub group_id: i64,
    pub user_id: i64,
    pub nickname: String,
    pub card: String,
    pub sex: crate::event::Sex,
    pub age: i32,
    pub join_time: i32,
    pub last_sent_time: i32,
    pub level: String,
    pub role: crate::event::Role,
    pub unfriendly: bool,
    pub card_changeable: bool,
}

/// get_group_honor_info 相关
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CurrentTalkative {
    pub user_id: i64,
    pub nickname: String,
    pub avatar: String,
    pub day_count: i32,
}

/// get_group_honor_info 相关
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HonorItem {
    pub user_id: i64,
    pub nickname: String,
    pub avatar: String,
    pub description: String,
}

/// Onebot Api 响应 sender 字段
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Sender {
    Group(crate::event::GroupSender),
    Private(crate::event::PrivateSender),
}

/// _get_model_show 响应数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelVariants {
    pub variants: Vec<Variant>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Variant {
    pub model_show: String,
    pub need_pay: bool,
}

/// get_online_clients 响应数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BotOnlineClients {
    pub clients: BotClients,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BotClients {
    pub app_id: i64,
    pub device_name: String,
    pub device_kind: String,
}

/// get_unidirectional_friend_list 响应数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnidirectionalFriendList {
    pub user_id: i64,
    pub nickname: String,
    pub source: String,
}

/// send_group_forward_msg/send_private_forward_msg 响应数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupMsgHistory {
    pub messages: crate::message::MessageVec,
}

/// send_group_forward_msg/send_private_forward_msg 响应数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ForwardMsgId {
    pub message_id: i32,
    pub forward_id: String,
}

/// ocr_image 响应数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OcrImages {
    pub texts: TextDetection,
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TextDetection {
    pub text: String,
    pub confidence: i32,
    pub coordinates: Vec<serde_json::Value>,
}

/// get_group_system_msg 响应数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupSystemMsg {
    pub invited_requests: Option<InvitedRequests>,
    pub join_requests: Option<JoinRequests>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InvitedRequests {
    pub request_id: i64,
    pub invitor_uin: i64,
    pub invitor_nick: String,
    pub group_id: i64,
    pub group_name: String,
    pub checked: bool,
    pub actor: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JoinRequests {
    pub request_id: i64,
    pub requester_uin: i64,
    pub requester_nick: String,
    pub message: String,
    pub group_id: i64,
    pub group_name: String,
    pub checked: bool,
    pub actor: i64,
}

/// get_essence_msg_list 响应数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EssenceMsgList {
    pub sender_id: i64,
    pub sender_nick: String,
    pub sender_time: i64,
    pub operator_id: i64,
    pub operator_nick: String,
    pub operator_time: i64,
    pub message_id: i32,
}

/// get_group_at_all_remain 响应数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupAtAllRemain {
    pub can_at_all: bool,
    pub remain_at_all_count_for_group: i16,
    pub remain_at_all_count_for_uin: i16,
}

/// _get_group_notice 响应数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupNotice {
    pub sender_id: i64,
    pub publish_time: i64,
    pub message: GroupNoticeMessage,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupNoticeMessage {
    pub text: String,
    pub images: Vec<GroupNoticeImage>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupNoticeImage {
    pub height: String,
    pub width: String,
    pub id: String,
}

// get_group_file_system_info 响应数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupFileSystemInfo {
    pub file_count: i32,
    pub limit_count: i32,
    pub used_space: i64,
    pub total_space: i64,
}

// get_group_root_files 响应数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupRootFiles {
    pub files: Files,
    pub folders: Folders,
}

// get_group_root_files 响应数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupFilesByFolder {
    pub files: Files,
    pub folders: Folders,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Files {
    pub group_id: i64,
    pub file_id: String,
    pub file_name: String,
    pub busid: i32,
    pub file_size: i64,
    pub upload_time: i64,
    pub dead_time: i64,
    pub modify_time: i64,
    pub download_times: i32,
    pub uploader: i64,
    pub uploader_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Folders {
    pub group_id: i64,
    pub folder_id: String,
    pub folder_name: i64,
    pub creator: i64,
    pub creator_name: String,
    pub total_file_count: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupFileUrl {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DownloadFiles {
    pub file: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UrlSafely {
    pub level: i8,
}


