use super::Matcher;
use crate::api_resp;
use crate::event::SelfId;
use colored::*;
use tracing::{event, Level};

macro_rules! no_resp_api {
    ($fn_name: ident, $param: ident: $param_type: ty) => {
        pub async fn $fn_name(&self, $param: $param_type) {
            if let Some(bot) = &self.bot {
                bot.$fn_name($param).await
            } else {
                event!(
                    Level::ERROR,
                    "Calling api {} {}",
                    stringify!($fn_name).blue(),
                    "with unbuilt matcher!".red()
                );
            }
        }
    };
    ($fn_name: ident, $($param: ident: $param_type: ty),*) => {
        pub async fn $fn_name(&self, $($param: $param_type,)*) {
            if let Some(bot) = &self.bot {
                bot.$fn_name($($param,)*).await
            } else {
                event!(
                    Level::ERROR,
                    "Calling api {} {}",
                    stringify!($fn_name).blue(),
                    "with unbuilt matcher!".red()
                );
            }
        }
    };
}

macro_rules! resp_api {
    ($fn_name: ident, $resp_data_type: ty) => {
        pub async fn $fn_name(&self) -> Option<$resp_data_type> {
            if let Some(bot) = &self.bot {
                bot.$fn_name().await
            } else {
                event!(
                    Level::ERROR,
                    "Calling api {} {}",
                    stringify!($fn_name).blue(),
                    "with unbuilt matcher!".red()
                );
                None
            }
        }
    };
    ($fn_name: ident, $resp_data_type: ty, $param: ident: $param_type: ty) => {
        pub async fn $fn_name(&self, $param: $param_type) -> Option<$resp_data_type> {
            if let Some(bot) = &self.bot {
                bot.$fn_name($param).await
            } else {
                event!(
                    Level::ERROR,
                    "Calling api {} {}",
                    stringify!($fn_name).blue(),
                    "with unbuilt matcher!".red()
                );
                None
            }
        }
    };
    ($fn_name: ident, $resp_data_type: ty, $($param: ident: $param_type: ty),*) => {
        pub async fn $fn_name(&self, $($param: $param_type,)*) -> Option<$resp_data_type> {
            if let Some(bot) = &self.bot {
                bot.$fn_name($($param,)*).await
            } else {
                event!(
                    Level::ERROR,
                    "Calling api {} {}",
                    stringify!($fn_name).blue(),
                    "with unbuilt matcher!".red()
                );
                None
            }
        }
    };
}

impl<E> Matcher<E>
where
    E: Clone + SelfId,
{
    /// 请求 Onebot Api，不等待 Onebot 返回
    pub async fn call_api(&self, api: crate::api::Api) {
        if let Some(bot) = &self.bot {
            bot.call_api(api).await;
        } else {
            event!(
                Level::ERROR,
                "{}",
                "Calling api with unbuilt matcher!".red()
            );
        }
    }

    /// 请求 Onebot Api，等待 Onebot 返回项（30s 后 timeout 返回 None）
    pub async fn call_api_resp(&self, api: crate::api::Api) -> Option<crate::api_resp::ApiResp> {
        if let Some(bot) = &self.bot {
            bot.call_api_resp(api).await
        } else {
            event!(
                Level::ERROR,
                "{}",
                "Calling api with unbuilt matcher!".red()
            );
            None
        }
    }

    // pub async fn delete_msg(&self, message_id: i32) {
    //     if let Some(bot) = &self.bot {
    //         bot.delete_msg(message_id).await
    //     } else {
    //         event!(
    //             Level::ERROR,
    //             "Calling api {} {}",
    //             "delete_msg".blue(),
    //             "with unbuilt matcher!".red()
    //         );
    //     }
    // }
    no_resp_api!(delete_msg, message_id: i32);
    no_resp_api!(send_like, user_id: i64, times: u8);
    no_resp_api!(
        set_group_kick,
        group_id: i64,
        user_id: i64,
        reject_add_request: bool
    );
    no_resp_api!(
        set_group_ban,
        group_id: i64,
        user_id: i64,
        duration: i64
    );
    no_resp_api!(
        set_group_anonymous_ban,
        group_id: i64,
        anonymous: crate::event::Anoymous,
        flag: String,
        duration: i64
    );
    no_resp_api!(set_group_whole_ban, group_id: i64, enable: bool);
    no_resp_api!(
        set_group_admin,
        group_id: i64,
        user_id: i64,
        enable: bool
    );
    no_resp_api!(set_group_anonymous, group_id: i64, enable: bool);
    no_resp_api!(
        set_group_card,
        group_id: i64,
        user_id: i64,
        card: String
    );
    no_resp_api!(set_group_name, group_id: i64, group_name: String);
    no_resp_api!(set_group_leave, group_id: i64, is_dismiss: bool);
    no_resp_api!(
        set_group_special_title,
        group_id: i64,
        user_id: i64,
        special_title: String,
        duration: i64
    );
    no_resp_api!(
        set_friend_add_request,
        flag: String,
        approve: bool,
        remark: String
    );
    no_resp_api!(
        set_group_add_request,
        flag: String,
        sub_type: String,
        approve: bool,
        reason: String
    );
    no_resp_api!(set_restart, delay: i64);
    no_resp_api!(set_qq_profile,
        nickname: Option<String>,
        company: Option<String>,
        email: Option<String>,
        college: Option<String>,
        personal_note: Option<String>
    );
    no_resp_api!(set_model_show,
        model: String,
        model_show: String
    );
    no_resp_api!(delete_friend,
        user_id:i64
    );
    no_resp_api!(delete_unidirectional_friend,
        user_id:i64
    );
    no_resp_api!(mark_msg_as_read,
        message_id:i32
    );
    no_resp_api!(set_group_portrait,
        group_id: i64,
        file: String,
        cache: i8
    );
    no_resp_api!(set_essence_msg,
        message_id: i32
    );
    no_resp_api!(delete_essence_msg,
        message_id: i32
    );
    no_resp_api!(send_group_sign,
        group_id: i32
    );
    no_resp_api!(send_group_notice,
        group_id: i64,
        content: String,
        image: String
    );
    no_resp_api!(delete_group_file,
        group_id: i64,
        file_id: String,
        busid: i32
    );
    no_resp_api!(create_group_file_folder,
        group_id: i64,
        name: String
    );
    no_resp_api!(delete_group_folder,
        group_id: i64,
        folder_id: String
    );
    resp_api!(
        send_msg,
        api_resp::MessageId,
        message_type: Option<String>,
        user_id: Option<String>,
        group_id: Option<String>,
        message: crate::message::MessageChain,
        auto_escape: bool
    );
    resp_api!(
        send_group_msg,
        api_resp::MessageId,
        group_id: i64,
        message: crate::message::MessageChain,
        auto_escape: bool
    );
    resp_api!(
        send_private_msg,
        api_resp::MessageId,
        user_id: i64,
        message: crate::message::MessageChain,
        auto_escape: bool
    );
    resp_api!(get_msg, api_resp::Message, message_id: i32);
    resp_api!(get_forward_msg, api_resp::Message, id: String);
    resp_api!(get_login_info, api_resp::LoginInfo);
    resp_api!(
        get_stranger_info,
        api_resp::StrangerInfo,
        user_id: i64,
        no_cache: bool
    );
    resp_api!(get_friend_list, Vec<api_resp::FriendListItem>);
    resp_api!(
        get_group_info,
        api_resp::GroupInfo,
        group_id: i64,
        no_cache: bool
    );
    resp_api!(get_group_list, Vec<api_resp::GroupListItem>);
    resp_api!(
        get_group_member_info,
        api_resp::GroupMemberInfo,
        group_id: i64,
        user_id: i64,
        no_cache: bool
    );
    resp_api!(
        get_group_member_list,
        Vec<api_resp::GroupMember>,
        group_id: i64
    );
    resp_api!(
        get_group_honor_info,
        api_resp::GroupHonorInfo,
        group_id: i64,
        type_: String
    );
    resp_api!(get_cookies, api_resp::Cookies, domain: String);
    resp_api!(get_csrf_token, api_resp::ScrfToken);
    resp_api!(get_credentials, api_resp::Credentials, domain: String);
    resp_api!(get_record, api_resp::File, file: String, out_format: String);
    resp_api!(get_image, api_resp::File, file: String);
    resp_api!(can_send_record, api_resp::SendCheck);
    resp_api!(can_send_image, api_resp::SendCheck);
    resp_api!(get_status, crate::event::Status);
    resp_api!(get_version_info, api_resp::VersionInfo);

    // pub async fn get_friend_list(&self) -> Option<Vec<crate::api_resp::FriendListItem>> {
    //     if let Some(bot) = &self.bot {
    //         bot.get_friend_list().await
    //     } else {
    //         event!(
    //             Level::ERROR,
    //             "Calling api {} {}",
    //             "get_friend_list".blue(),
    //             "with unbuilt matcher!".red()
    //         );
    //         None
    //     }
    // }

    // pub async fn get_group_list(&self) -> Option<Vec<crate::api_resp::GroupListItem>> {
    //     if let Some(bot) = &self.bot {
    //         bot.get_group_list().await
    //     } else {
    //         event!(
    //             Level::ERROR,
    //             "{}",
    //             "Calling api with unbuilt matcher!".red()
    //         );
    //         None
    //     }
    // }
    resp_api!(get_model_show,
        api_resp::ModelVariants,
        model:String
    );
    resp_api!(get_online_clients,
        api_resp::BotOnlineClients,
        no_cache: bool
    );
    resp_api!(get_unidirectional_friend_list,
        Vec<api_resp::UnidirectionalFriendList>
    );
    resp_api!(send_group_forward_msg,
        api_resp::ForwardMsgId,
        group_id: i64,
        messages: crate::message::MessageChain
    );
    resp_api!(send_private_forward_msg,
        api_resp::ForwardMsgId,
        user_id: i64,
        messages: crate::message::MessageChain
    );
    resp_api!(get_group_msg_history,
        api_resp::GroupMsgHistory,
        group_id: i64,
        message_seq: String
    );
    resp_api!(ocr_image,
        api_resp::OcrImages,
        image: String
    );
    resp_api!(get_group_system_msg,
        api_resp::GroupSystemMsg
    );
    resp_api!(get_essence_msg_list,
        Vec<api_resp::EssenceMsgList>,
        group_id:i64
    );
    resp_api!(get_group_at_all_remain,
        api_resp::GroupAtAllRemain,
        group_id:i64
    );
    resp_api!(get_group_notice,
        api_resp::GroupNotice,
        group_id:i64
    );
    resp_api!(upload_group_file,
        crate::ApiResp,
        group_id:i64,
        file:String,
        name:String,
        folder:Option<String>
    );
    resp_api!(upload_private_file,
        crate::ApiResp,
        user_id:i64,
        file:String,
        name:String
    );

    resp_api!(download_file,
        api_resp::DownloadFiles,
        url:String,
        thread_count:i32,
        headers:Vec<String>
    );
    resp_api!(get_group_file_system_info,
        api_resp::GroupFileSystemInfo,
        group_id:i64
    );
    resp_api!(get_group_root_files,
        api_resp::GroupRootFiles,
        group_id:i64
    );
    resp_api!(get_group_files_by_folder,
        api_resp::GroupFilesByFolder,
        group_id:i64,
        folder_id:String
    );
    resp_api!(get_group_file_url,
        api_resp::GroupRootFiles,
        group_id:i64,
        file_id:String,
        busid:i32
    );
    resp_api!(check_url_safely,
        api_resp::UrlSafely,
        url:String
    );
}
