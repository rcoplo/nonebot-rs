use crate::{api, api_resp, ApiResp, RespData};

macro_rules! no_resp_api {
    ($fn_name: ident, $struct_name: tt, $param: ident: $param_type: ty) => {
        pub async fn $fn_name(&self, $param: $param_type) {
            self.call_api(api::Api::$fn_name(api::$struct_name { $param: $param }))
                .await;
        }
    };
    ($fn_name: ident, $struct_name: tt, $($param: ident: $param_type: ty),*) => {
        pub async fn $fn_name(&self, $($param: $param_type,)*) {
            self.call_api(api::Api::$fn_name(api::$struct_name {
                $($param: $param,)*
            })).await;
        }
    };
}

macro_rules! resp_api {
    ($fn_name: ident,$resp_data: tt, $resp_data_type: ty) => {
        pub async fn $fn_name(&self) -> Option<$resp_data_type> {
            let resp = self.call_api_resp(api::Api::$fn_name()).await;
            if let RespData::$resp_data(d) = resp.unwrap().data {
                Some(d)
            } else {
                None
            }
        }
    };
    ($fn_name: ident, $struct_name: tt, $resp_data: tt, $resp_data_type: ty, $param: ident: $param_type: ty) => {
        pub async fn $fn_name(&self, $param: $param_type) -> Option<$resp_data_type> {
            let resp = self
                .call_api_resp(api::Api::$fn_name(api::$struct_name { $param: $param }))
                .await;
            if let RespData::$resp_data(d) = resp.unwrap().data {
                Some(d)
            } else {
                None
            }
        }
    };
    ($fn_name: ident, $struct_name: tt, $resp_data: tt, $resp_data_type: ty, $($param: ident: $param_type: ty),*) => {
        pub async fn $fn_name(&self, $($param: $param_type,)*) -> Option<$resp_data_type> {
            let resp = self
                .call_api_resp(api::Api::$fn_name(api::$struct_name {
                    $($param: $param,)*
                }))
                .await;
            if let RespData::$resp_data(d) = resp.unwrap().data {
                Some(d)
            } else {
                None
            }
        }
    };
}

impl super::Bot {
    // pub async fn delete_msg(&self, message_id: i32) {
    //     self.call_api(Api::delete_msg(api::DeleteMsg {
    //         message_id: message_id,
    //     }))
    //     .await;
    // }
    no_resp_api!(delete_msg, DeleteMsg, message_id: i32);
    no_resp_api!(send_like, SendLike, user_id: i64, times: u8);
    no_resp_api!(
        set_group_kick,
        SetGroupKick,
        group_id: i64,
        user_id: i64,
        reject_add_request: bool
    );
    no_resp_api!(
        set_group_ban,
        SetGroupBan,
        group_id: i64,
        user_id: i64,
        duration: i64
    );
    no_resp_api!(
        set_group_anonymous_ban,
        SetGroupAnonymousBan,
        group_id: i64,
        anonymous: crate::event::Anoymous,
        flag: String,
        duration: i64
    );
    no_resp_api!(
        set_group_whole_ban,
        SetGroupWholeBan,
        group_id: i64,
        enable: bool
    );
    no_resp_api!(
        set_group_admin,
        SetGroupAdmin,
        group_id: i64,
        user_id: i64,
        enable: bool
    );
    no_resp_api!(
        set_group_anonymous,
        SetGroupAnonymous,
        group_id: i64,
        enable: bool
    );
    no_resp_api!(
        set_group_card,
        SetGroupCard,
        group_id: i64,
        user_id: i64,
        card: String
    );
    no_resp_api!(
        set_group_name,
        SetGroupName,
        group_id: i64,
        group_name: String
    );
    no_resp_api!(
        set_group_leave,
        SetGroupLeave,
        group_id: i64,
        is_dismiss: bool
    );
    no_resp_api!(
        set_group_special_title,
        SetGroupSpecialTitle,
        group_id: i64,
        user_id: i64,
        special_title: String,
        duration: i64
    );
    no_resp_api!(
        set_friend_add_request,
        SetFriendAddRequest,
        flag: String,
        approve: bool,
        remark: String
    );
    no_resp_api!(
        set_group_add_request,
        SetGroupAddRequest,
        flag: String,
        sub_type: String,
        approve: bool,
        reason: String
    );
    no_resp_api!(set_restart, SetRestart, delay: i64);
    no_resp_api!(set_qq_profile,
        SetQqProfile,
        nickname: Option<String>,
        company: Option<String>,
        email: Option<String>,
        college: Option<String>,
        personal_note: Option<String>
    );
    no_resp_api!(set_model_show,
        SetModelShow,
        model: String,
        model_show: String
    );
    no_resp_api!(delete_friend,
        DeleteFriend,
        user_id:i64
    );
    no_resp_api!(delete_unidirectional_friend,
        DeleteUnidirectionalFriend,
        user_id:i64
    );
    no_resp_api!(mark_msg_as_read,
        MarkMsgAsRead,
        message_id:i32
    );
    no_resp_api!(set_group_portrait,
        SetGroupPortrait,
        group_id: i64,
        file: String,
        cache: i8
    );
    no_resp_api!(set_essence_msg,
        SetEssenceMsg,
        message_id: i32
    );
    no_resp_api!(delete_essence_msg,
        DeleteEssenceMsg,
        message_id: i32
    );
    no_resp_api!(send_group_sign,
        SendGroupSign,
        message_id: i32
    );
    no_resp_api!(send_group_notice,
        SendGroupNotice,
        group_id: i64,
        content: String,
        image: String
    );
    no_resp_api!(delete_group_file,
        DeleteGroupFile,
        group_id: i64,
        file_id: String,
        busid: i32
    );
    pub async fn create_group_file_folder(&self, group_id: i64, name: String) {
        self.call_api(api::Api::create_group_file_folder(api::CreateGroupFileFolder {
            group_id,
            name,
            parent_id: "/".to_string(),
        }))
            .await;
    }
    no_resp_api!(delete_group_folder,
        DeleteGroupFolder,
        group_id: i64,
        folder_id: String
    );

    // ??????
    // pub async fn get_msg(&self, message_id: i32) -> Option<api_resp::Message> {
    //     let resp = self
    //         .call_api_resp(Api::get_msg(api::GetMsg {
    //             message_id: message_id,
    //         }))
    //         .await;
    //     if let RespData::Message(m) = resp.unwrap().data {
    //         Some(m)
    //     } else {
    //         None
    //     }
    // }
    resp_api!(
        send_msg,
        SendMsg,
        MessageId,
        api_resp::MessageId,
        message_type: Option<String>,
        user_id: Option<String>,
        group_id: Option<String>,
        message: crate::message::MessageChain,
        auto_escape: bool
    );
    resp_api!(
        send_group_msg,
        SendGroupMsg,
        MessageId,
        api_resp::MessageId,
        group_id: i64,
        message: crate::message::MessageChain,
        auto_escape: bool
    );
    resp_api!(
        send_private_msg,
        SendPrivateMsg,
        MessageId,
        api_resp::MessageId,
        user_id: i64,
        message: crate::message::MessageChain,
        auto_escape: bool
    );
    resp_api!(get_msg, GetMsg, Message, api_resp::Message, message_id: i32);
    resp_api!(
        get_forward_msg,
        GetForwardMsg,
        Message,
        api_resp::Message,
        id: String
    );
    resp_api!(get_login_info, LoginInfo, api_resp::LoginInfo);
    resp_api!(
        get_stranger_info,
        GetStrangerInfo,
        StrangerInfo,
        api_resp::StrangerInfo,
        user_id: i64,
        no_cache: bool
    );
    resp_api!(get_friend_list, FriendList, Vec<api_resp::FriendListItem>);
    resp_api!(
        get_group_info,
        GetGroupInfo,
        GroupInfo,
        api_resp::GroupInfo,
        group_id: i64,
        no_cache: bool
    );
    resp_api!(get_group_list, GroupList, Vec<api_resp::GroupListItem>);
    resp_api!(
        get_group_member_info,
        GetGroupMemberInfo,
        GroupMemberInfo,
        api_resp::GroupMemberInfo,
        group_id: i64,
        user_id: i64,
        no_cache: bool
    );
    resp_api!(
        get_group_member_list,
        GetGroupMemberList,
        GroupMemberList,
        Vec<api_resp::GroupMember>,
        group_id: i64
    );
    resp_api!(
        get_group_honor_info,
        GetGroupHonorInfo,
        GroupHonorInfo,
        api_resp::GroupHonorInfo,
        group_id: i64,
        type_: String
    );
    resp_api!(
        get_cookies,
        GetCookies,
        Cookies,
        api_resp::Cookies,
        domain: String
    );
    resp_api!(get_csrf_token, ScrfToken, api_resp::ScrfToken);
    resp_api!(
        get_credentials,
        GetCookies,
        Credentials,
        api_resp::Credentials,
        domain: String
    );
    resp_api!(
        get_record,
        GetRecord,
        File,
        api_resp::File,
        file: String,
        out_format: String
    );
    resp_api!(get_image, GetImage, File, api_resp::File, file: String);
    resp_api!(can_send_record, SendCheck, api_resp::SendCheck);
    resp_api!(can_send_image, SendCheck, api_resp::SendCheck);
    resp_api!(get_status, Status, crate::event::Status);
    resp_api!(get_version_info, VersionInfo, api_resp::VersionInfo);
    resp_api!(get_model_show,
        GetModelShow,
        ModelVariants,
        api_resp::ModelVariants,
        model:String
    );
    resp_api!(get_online_clients,
        GetOnlineClients,
        BotOnlineClients,
        api_resp::BotOnlineClients,
        no_cache:bool
    );
    resp_api!(get_unidirectional_friend_list,
        UnidirectionalFriendList,
       Vec<api_resp::UnidirectionalFriendList>
    );
    resp_api!(send_group_forward_msg,
        SendGroupForwardMsg,
        ForwardMsgId,
        api_resp::ForwardMsgId,
        group_id: i64,
        messages: crate::message::MessageChain
    );
    resp_api!(send_private_forward_msg,
        SendPrivateForwardMsg,
        ForwardMsgId,
        api_resp::ForwardMsgId,
        user_id: i64,
        messages: crate::message::MessageChain
    );
    resp_api!(get_group_msg_history,
        GetGroupMsgHistory,
        GroupMsgHistory,
        api_resp::GroupMsgHistory,
        group_id: i64,
        message_seq: String
    );
    resp_api!(ocr_image,
        OcrImage,
        OcrImage,
        api_resp::OcrImages,
        image: String
    );
    resp_api!(get_group_system_msg,
        GroupSystemMsg,
        api_resp::GroupSystemMsg
    );
    resp_api!(get_essence_msg_list,
        GetEssenceMsgList,
        EssenceMsgList,
        Vec<api_resp::EssenceMsgList>,
        group_id:i64
    );
    resp_api!(get_group_at_all_remain,
        GetGroupAtAllRemain,
        GroupAtAllRemain,
        api_resp::GroupAtAllRemain,
        group_id:i64
    );
    resp_api!(get_group_notice,
        GetGroupNotice,
        GroupNotice,
        api_resp::GroupNotice,
        group_id:i64
    );

    pub async fn upload_group_file(&self, group_id: i64, file: String, name: String, folder: Option<String>) -> Option<ApiResp> {
        let file = match self._download_file(file).await {
            None => return None,
            Some(file) => {
                file
            }
        };
        let resp = self.call_api_resp(api::Api::upload_group_file(api::UploadGroupFile {
            group_id,
            file,
            name,
            folder,
        })).await;
        resp
    }

    pub async fn upload_private_file(&self, user_id: i64, file: String, name: String) -> Option<ApiResp> {
        let file = match self._download_file(file).await {
            None => return None,
            Some(file) => { file }
        };
        let resp = self.call_api_resp(api::Api::upload_private_file(api::UploadPrivateFile {
            user_id,
            file,
            name,
        })).await;
        resp
    }
    async fn _download_file(&self, file: String) -> Option<String> {
        if file.starts_with("http") {
            let headers = vec![
                "User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Safari/537.36 Edg/112.0.1722.34".to_string()
            ];
            match self.download_file(file, 8, headers).await {
                None => None,
                Some(file) => {
                    Some(file.file)
                }
            }
        } else {
            Some(file)
        }
    }
    resp_api!(download_file,
        DownloadFile,
        DownloadFile,
        api_resp::DownloadFiles,
        url:String,
        thread_count:i32,
        headers:Vec<String>
    );
    resp_api!(get_group_file_system_info,
        GetGroupFileSystemInfo,
        GroupFileSystemInfo,
        api_resp::GroupFileSystemInfo,
        group_id:i64
    );
    resp_api!(get_group_root_files,
        GetGroupRootFiles,
        GroupRootFiles,
        api_resp::GroupRootFiles,
        group_id:i64
    );
    resp_api!(get_group_files_by_folder,
        GetGroupFilesByFolder,
        GroupFilesByFolder,
        api_resp::GroupFilesByFolder,
        group_id:i64,
        folder_id:String
    );
    resp_api!(get_group_file_url,
        GetGroupFileUrl,
        GroupRootFiles,
        api_resp::GroupRootFiles,
        group_id:i64,
        file_id:String,
        busid:i32
    );
    resp_api!(check_url_safely,
        CheckUrlSafely,
        UrlSafely,
        api_resp::UrlSafely,
        url:String
    );
}
