
use serde::{Deserialize, Serialize};

/// Onebot Api ����
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "action")]
pub enum Api {
    #[serde(rename = "send_private_msg")]
    SendPrivateMsg {
        params: SendPrivateMsg,
        echo: String,
    },
    #[serde(rename = "send_group_msg")]
    SendGroupMsg { params: SendGroupMsg, echo: String },
    #[serde(rename = "send_msg")]
    SendMsg { params: SendMsg, echo: String },
    #[serde(rename = "delete_msg")]
    DeleteMsg { params: DeleteMsg, echo: String },
    #[serde(rename = "get_msg")]
    GetMsg { params: GetMsg, echo: String },
    #[serde(rename = "get_forward_msg")]
    GetForwardMsg { params: GetForwardMsg, echo: String },
    #[serde(rename = "send_like")]
    SendLike { params: SendLike, echo: String },
    #[serde(rename = "set_group_kick")]
    SetGroupKick { params: SetGroupKick, echo: String },
    #[serde(rename = "set_group_ban")]
    SetGroupBan { params: SetGroupBan, echo: String },
    #[serde(rename = "set_group_anonymous_ban")]
    SetGroupAnonymousBan {
        params: SetGroupAnonymousBan,
        echo: String,
    },
    #[serde(rename = "set_group_whole_ban")]
    SetGroupWholeBan {
        params: SetGroupWholeBan,
        echo: String,
    },
    #[serde(rename = "set_group_admin")]
    SetGroupAdmin { params: SetGroupAdmin, echo: String },
    #[serde(rename = "set_group_anonymous")]
    SetGroupAnonymous {
        params: SetGroupAnonymous,
        echo: String,
    },
    #[serde(rename = "set_group_card")]
    SetGroupCard { params: SetGroupCard, echo: String },
    #[serde(rename = "set_group_name")]
    SetGroupName { params: SetGroupName, echo: String },
    #[serde(rename = "set_group_leave")]
    SetGroupLeave { params: SetGroupLeave, echo: String },
    #[serde(rename = "set_group_special_title")]
    SetGroupSpecialTitle {
        params: SetGroupSpecialTitle,
        echo: String,
    },
    #[serde(rename = "set_friend_add_request")]
    SetFriendAddRequest {
        params: SetFriendAddRequest,
        echo: String,
    },
    #[serde(rename = "set_group_add_request")]
    SetGroupAddRequest {
        params: SetGroupAddRequest,
        echo: String,
    },
    #[serde(rename = "get_login_info")]
    GetLoginInfo { params: Option<i8>, echo: String },
    #[serde(rename = "get_stranger_info")]
    GetStrangerInfo {
        params: GetStrangerInfo,
        echo: String,
    },
    #[serde(rename = "get_friend_list")]
    GetFriendList { params: Option<i8>, echo: String },
    #[serde(rename = "get_group_info")]
    GetGroupInfo { params: GetGroupInfo, echo: String },
    #[serde(rename = "get_group_list")]
    GetGroupList { params: Option<i8>, echo: String },
    #[serde(rename = "get_group_member_info")]
    GetGroupMemberInfo {
        params: GetGroupMemberInfo,
        echo: String,
    },
    #[serde(rename = "get_group_member_list")]
    GetGroupMemberList {
        params: GetGroupMemberList,
        echo: String,
    },
    #[serde(rename = "get_group_honor_info")]
    GetGroupHonorInfo {
        params: GetGroupHonorInfo,
        echo: String,
    },
    #[serde(rename = "get_cookies")]
    GetCookies { params: GetCookies, echo: String },
    #[serde(rename = "get_csrf_token")]
    GetCsrfToken { params: Option<i8>, echo: String },
    #[serde(rename = "get_credentials")]
    GetCredentials { params: GetCookies, echo: String },
    #[serde(rename = "get_record")]
    GetRecord { params: GetRecord, echo: String },
    #[serde(rename = "get_image")]
    GetImage { params: GetImage, echo: String },
    #[serde(rename = "can_send_image")]
    CanSendImage { params: Option<i8>, echo: String },
    #[serde(rename = "can_send_record")]
    CanSendRecord { params: Option<i8>, echo: String },
    #[serde(rename = "get_status")]
    GetStatus { params: Option<i8>, echo: String },
    #[serde(rename = "get_version_info")]
    GetVersionInfo { params: Option<i8>, echo: String },
    /// go-cqhttp 该 API 由于技术原因，自 1.0.0 版本已被移除，目前暂时没有再加入的计划  [#1230](https://github.com/Mrs4s/go-cqhttp/issues/1230)
    #[serde(rename = "set_restart")]
    SetRestart { params: SetRestart, echo: String },
    #[serde(rename = "clean_cache")]
    CleanCache { params: Option<i8>, echo: String },

    /// 以下是程序没有的api  [go-cqhttp](https://docs.go-cqhttp.org/api/)
    #[serde(rename = "set_qq_profile")]
    SetQqProfile { params: SetQqProfile, echo: String },
    #[serde(rename = "_get_model_show")]
    GetModelShow { params: GetModelShow, echo: String },
    /// [例子](https://github.com/Mrs4s/go-cqhttp/pull/872#issuecomment-831180149)
    #[serde(rename = "_set_model_show")]
    SetModelShow { params: SetModelShow, echo: String },
    #[serde(rename = "get_online_clients")]
    GetOnlineClients { params: GetOnlineClients, echo: String },
    #[serde(rename = "get_unidirectional_friend_list")]
    GetUnidirectionalFriendList { params: Option<i8>, echo: String },
    #[serde(rename = "delete_friend")]
    DeleteFriend { params: DeleteFriend, echo: String },
    #[serde(rename = "delete_unidirectional_friend")]
    DeleteUnidirectionalFriend { params: DeleteUnidirectionalFriend, echo: String },
    #[serde(rename = "mark_msg_as_read")]
    MarkMsgAsRead { params: MarkMsgAsRead, echo: String },
    #[serde(rename = "send_group_forward_msg")]
    SendGroupForwardMsg { params: SendGroupForwardMsg, echo: String },
    #[serde(rename = "send_private_forward_msg")]
    SendPrivateForwardMsg { params: SendPrivateForwardMsg, echo: String },
    #[serde(rename = "get_group_msg_history")]
    GetGroupMsgHistory { params: GetGroupMsgHistory, echo: String },
    #[serde(rename = "ocr_image")]
    OcrImage { params: OcrImage, echo: String },
    #[serde(rename = "get_group_system_msg")]
    GetGroupSystemMsg { params: Option<i8>, echo: String },
    #[serde(rename = "get_essence_msg_list")]
    GetEssenceMsgList { params: GetEssenceMsgList, echo: String },
    #[serde(rename = "get_group_at_all_remain")]
    GetGroupAtAllRemain { params: GetGroupAtAllRemain, echo: String },
    #[serde(rename = "set_group_portrait")]
    SetGroupPortrait { params: SetGroupPortrait, echo: String },
    #[serde(rename = "set_essence_msg")]
    SetEssenceMsg { params: SetEssenceMsg, echo: String },
    #[serde(rename = "delete_essence_msg")]
    DeleteEssenceMsg { params: DeleteEssenceMsg, echo: String },
    #[serde(rename = "send_group_sign")]
    SendGroupSign { params: SendGroupSign, echo: String },
    #[serde(rename = "_send_group_notice")]
    SendGroupNotice { params: SendGroupNotice, echo: String },
    #[serde(rename = "_get_group_notice")]
    GetGroupNotice { params: GetGroupNotice, echo: String },

    /// 操作群文件api
    #[serde(rename = "upload_group_file")]
    UploadGroupFile { params: UploadGroupFile, echo: String },
    #[serde(rename = "delete_group_file")]
    DeleteGroupFile { params: DeleteGroupFile, echo: String },
    #[serde(rename = "create_group_file_folder")]
    CreateGroupFileFolder { params: CreateGroupFileFolder, echo: String },
    #[serde(rename = "delete_group_folder")]
    DeleteGroupFolder { params: DeleteGroupFolder, echo: String },
    #[serde(rename = "get_group_file_system_info")]
    GetGroupFileSystemInfo { params: GetGroupFileSystemInfo, echo: String },
    #[serde(rename = "get_group_root_files")]
    GetGroupRootFiles { params: GetGroupRootFiles, echo: String },
    #[serde(rename = "get_group_files_by_folder")]
    GetGroupFilesByFolder { params: GetGroupFilesByFolder, echo: String },
    #[serde(rename = "get_group_file_url")]
    GetGroupFileUrl { params: GetGroupFileUrl, echo: String },
    #[serde(rename = "upload_private_file")]
    UploadPrivateFile { params: UploadPrivateFile, echo: String },
    #[serde(rename = "download_file")]
    DownloadFile { params: DownloadFile, echo: String },

    #[serde(rename = "check_url_safely")]
    CheckUrlSafely { params: CheckUrlSafely, echo: String },

}

macro_rules! echos {
    ($($x: tt),*) => {
        pub fn get_echo(&self) -> String {
            match self {
                $(Api::$x {
                    params: _,
                    echo: echo,
                } => echo.clone(),)*
            }
        }
    };
}

macro_rules! no_params_builder {
    ($(($fn_name: ident, $api_type: tt)),*) => {
        $(pub fn $fn_name() -> Api {
            Api::$api_type {
                params: None,
                echo: format!("{}-{}", stringify!($api_type), crate::utils::timestamp()),
            }
        })*
    };
}

macro_rules! params_builder {
    ($(($fn_name: ident, $api_type: tt)),*) => {
        $(pub fn $fn_name(params: $api_type) -> Api {
            Api::$api_type {
                params: params,
                echo: format!("{}-{}", stringify!($api_type), crate::utils::timestamp()),
            }
        })*
    };
}

impl Api {
    // Api::SendPrivateMsg {
    //     params: _,
    //     echo: echo,
    // } => echo.clone(),
    echos!(
        SendPrivateMsg,
        SendGroupMsg,
        SendMsg,
        DeleteMsg,
        GetMsg,
        GetForwardMsg,
        SendLike,
        SetGroupKick,
        SetGroupBan,
        SetGroupAnonymousBan,
        SetGroupWholeBan,
        SetGroupAdmin,
        SetGroupAnonymous,
        SetGroupCard,
        SetGroupName,
        SetGroupLeave,
        SetGroupSpecialTitle,
        SetFriendAddRequest,
        SetGroupAddRequest,
        GetLoginInfo,
        GetStrangerInfo,
        GetFriendList,
        GetGroupInfo,
        GetGroupList,
        GetGroupMemberInfo,
        GetGroupMemberList,
        GetGroupHonorInfo,
        GetCookies,
        GetCsrfToken,
        GetCredentials,
        GetRecord,
        GetImage,
        CanSendImage,
        CanSendRecord,
        GetStatus,
        GetVersionInfo,
        SetRestart,
        CleanCache,
        SetQqProfile,
        GetModelShow,
        SetModelShow,
        GetOnlineClients,
        GetUnidirectionalFriendList,
        DeleteFriend,
        DeleteUnidirectionalFriend,
        MarkMsgAsRead,
        SendGroupForwardMsg,
        SendPrivateForwardMsg,
        GetGroupMsgHistory,
        OcrImage,
        GetGroupSystemMsg,
        GetEssenceMsgList,
        GetGroupAtAllRemain,
        SetGroupPortrait,
        SetEssenceMsg,
        DeleteEssenceMsg,
        SendGroupSign,
        SendGroupNotice,
        GetGroupNotice,
        UploadGroupFile,
        DeleteGroupFile,
        CreateGroupFileFolder,
        DeleteGroupFolder,
        GetGroupFileSystemInfo,
        GetGroupRootFiles,
        GetGroupFilesByFolder,
        GetGroupFileUrl,
        UploadPrivateFile,
        DownloadFile,
        CheckUrlSafely
    );

    // pub fn get_group_list() -> Api {
    //     Api::GetGroupList {
    //         params: None,
    //         echo: format!("{},{}", "GetGroupList", crate::utils::timestamp()),
    //     }
    // }
    no_params_builder!(
        (get_login_info, GetLoginInfo),
        (get_friend_list, GetFriendList),
        (get_group_list, GetGroupList),
        (get_csrf_token, GetCsrfToken),
        (can_send_image, CanSendImage),
        (can_send_record, CanSendRecord),
        (get_status, GetStatus),
        (get_version_info, GetVersionInfo),
        (clean_cache, CleanCache),
        (get_unidirectional_friend_list, GetUnidirectionalFriendList),
        (get_group_system_msg, GetGroupSystemMsg)
    );

    // pub fn send_private_msg(params: SendPrivateMsg) -> Api {
    //     Api::SendPrivateMsg {
    //         params: params,
    //         echo: format!("{}-{}", "SendGroupMsg", crate::utils::timestamp()),
    //     }
    // }
    params_builder!(
        (send_private_msg, SendPrivateMsg),
        (send_group_msg, SendGroupMsg),
        (send_msg, SendMsg),
        (delete_msg, DeleteMsg),
        (get_msg, GetMsg),
        (get_forward_msg, GetForwardMsg),
        (send_like, SendLike),
        (set_group_kick, SetGroupKick),
        (set_group_ban, SetGroupBan),
        (set_group_anonymous_ban, SetGroupAnonymousBan),
        (set_group_whole_ban, SetGroupWholeBan),
        (set_group_admin, SetGroupAdmin),
        (set_group_anonymous, SetGroupAnonymous),
        (set_group_card, SetGroupCard),
        (set_group_name, SetGroupName),
        (set_group_leave, SetGroupLeave),
        (set_group_special_title, SetGroupSpecialTitle),
        (set_friend_add_request, SetFriendAddRequest),
        (set_group_add_request, SetGroupAddRequest),
        (get_stranger_info, GetStrangerInfo),
        (get_group_info, GetGroupInfo),
        (get_group_member_info, GetGroupMemberInfo),
        (get_group_member_list, GetGroupMemberList),
        (get_group_honor_info, GetGroupHonorInfo),
        (get_cookies, GetCookies),
        (get_credentials, GetCookies),
        (get_record, GetRecord),
        (get_image, GetImage),
        (set_restart, SetRestart),
        (set_qq_profile, SetQqProfile),
        (get_model_show, GetModelShow),
        (set_model_show, SetModelShow),
        (get_online_clients, GetOnlineClients),
        (delete_friend, DeleteFriend),
        (delete_unidirectional_friend, DeleteUnidirectionalFriend),
        (mark_msg_as_read, MarkMsgAsRead),
        (send_group_forward_msg, SendGroupForwardMsg),
        (send_private_forward_msg, SendPrivateForwardMsg),
        (get_group_msg_history, GetGroupMsgHistory),
        (ocr_image, OcrImage),
        (get_essence_msg_list, GetEssenceMsgList),
        (get_group_at_all_remain, GetGroupAtAllRemain),
        (set_group_portrait, SetGroupPortrait),
        (set_essence_msg, SetEssenceMsg),
        (delete_essence_msg, DeleteEssenceMsg),
        (send_group_sign, SendGroupSign),
        (send_group_notice, SendGroupNotice),
        (get_group_notice, GetGroupNotice),
        (upload_group_file, UploadGroupFile),
        (delete_group_file, DeleteGroupFile),
        (create_group_file_folder, CreateGroupFileFolder),
        (delete_group_folder, DeleteGroupFolder),
        (get_group_file_system_info, GetGroupFileSystemInfo),
        (get_group_root_files, GetGroupRootFiles),
        (get_group_files_by_folder, GetGroupFilesByFolder),
        (get_group_file_url, GetGroupFileUrl),
        (upload_private_file, UploadPrivateFile),
        (download_file, DownloadFile),
        (check_url_safely, CheckUrlSafely)
    );
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendPrivateMsg {
    pub user_id: i64,
    pub message: crate::message::MessageChain,
    pub auto_escape: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendGroupMsg {
    pub group_id: i64,
    pub message: crate::message::MessageChain,
    pub auto_escape: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendMsg {
    pub message_type: Option<String>,
    pub user_id: Option<String>,
    pub group_id: Option<String>,
    pub message: crate::message::MessageChain,
    pub auto_escape: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeleteMsg {
    pub message_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetMsg {
    pub message_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetForwardMsg {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendLike {
    pub user_id: i64,
    pub times: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetGroupKick {
    pub group_id: i64,
    pub user_id: i64,
    pub reject_add_request: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetGroupBan {
    pub group_id: i64,
    pub user_id: i64,
    pub duration: i64, // ???????????????0??????????
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetGroupAnonymousBan {
    pub group_id: i64,
    pub anonymous: crate::event::Anoymous,
    pub flag: String,
    pub duration: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetGroupWholeBan {
    pub group_id: i64,
    pub enable: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetGroupAdmin {
    pub group_id: i64,
    pub user_id: i64,
    pub enable: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetGroupAnonymous {
    pub group_id: i64,
    pub enable: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetGroupCard {
    pub group_id: i64,
    pub user_id: i64,
    pub card: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetGroupName {
    pub group_id: i64,
    pub group_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetGroupLeave {
    pub group_id: i64,
    pub is_dismiss: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetGroupSpecialTitle {
    pub group_id: i64,
    pub user_id: i64,
    pub special_title: String,
    pub duration: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetFriendAddRequest {
    pub flag: String,
    pub approve: bool,
    pub remark: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetGroupAddRequest {
    pub flag: String,
    pub sub_type: String,
    pub approve: bool,
    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetStrangerInfo {
    pub user_id: i64,
    pub no_cache: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetGroupInfo {
    pub group_id: i64,
    pub no_cache: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetGroupMemberInfo {
    pub group_id: i64,
    pub user_id: i64,
    pub no_cache: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetGroupMemberList {
    pub group_id: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetGroupHonorInfo {
    pub group_id: i64,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetCookies {
    pub domain: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetRecord {
    pub file: String,
    pub out_format: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetImage {
    pub file: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetRestart {
    pub delay: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetQqProfile {
    pub nickname: Option<String>,
    pub company: Option<String>,
    pub email: Option<String>,
    pub college: Option<String>,
    pub personal_note: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetModelShow {
    pub model: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetModelShow {
    pub model: String,
    pub model_show: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetOnlineClients {
    pub no_cache: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeleteFriend {
    pub user_id: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeleteUnidirectionalFriend {
    pub user_id: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MarkMsgAsRead {
    pub message_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendGroupForwardMsg {
    pub group_id: i64,
    pub messages: crate::message::MessageChain,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendPrivateForwardMsg {
    pub user_id: i64,
    pub messages: crate::message::MessageChain,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetGroupMsgHistory {
    pub message_seq: String,
    pub group_id: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OcrImage {
    pub image: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetEssenceMsgList {
    pub group_id: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetGroupAtAllRemain {
    pub group_id: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetGroupPortrait {
    pub group_id: i64,
    pub file: String,
    pub cache: i8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetEssenceMsg {
    pub message_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeleteEssenceMsg {
    pub message_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendGroupSign {
    pub message_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendGroupNotice {
    pub group_id: i64,
    pub content: String,
    pub image: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetGroupNotice {
    pub group_id: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UploadGroupFile {
    pub group_id: i64,
    pub file: String,
    pub name: String,
    pub folder: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeleteGroupFile {
    pub group_id: i64,
    pub file_id: String,
    pub busid: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateGroupFileFolder {
    pub group_id: i64,
    pub name: String,
    ///仅能为 `/`
    pub parent_id: String,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeleteGroupFolder {
    pub group_id: i64,
    pub folder_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetGroupFileSystemInfo {
    pub group_id: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetGroupRootFiles {
    pub group_id: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetGroupFilesByFolder {
    pub group_id: i64,
    pub folder_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetGroupFileUrl {
    pub group_id: i64,
    pub file_id: String,
    pub busid: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UploadPrivateFile {
    pub user_id: i64,
    pub file: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DownloadFile {
    pub url: String,
    pub thread_count: i32,
    pub headers: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CheckUrlSafely {
    pub url: String,
}

