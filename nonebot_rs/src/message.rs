use serde::{Deserialize, Serialize};


/// Onebot 协议消息定义
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type", content = "data")]
pub enum Message {
    /// 纯文本
    #[serde(rename = "text")]
    Text {
        /// 纯文本内容
        text: String,
    },

    /// QQ 表情
    #[serde(rename = "face")]
    Face {
        /// QQ 表情 ID
        id: String,
    },

    /// 图片
    #[serde(rename = "image")]
    Image {
        /// 图片文件名
        file: String,
        /// 图片类型 flash 闪照
        #[serde(rename = "type")]
        ty: Option<String>,
        /// 图片 URL
        url: Option<String>,
        /// 是否使用缓存文件 1|0
        cache: Option<u8>,
        /// 是否使用代理 1|0
        proxy: Option<u8>,
        /// 网络文件下载超时 单位秒
        timeout: Option<i64>,
    },

    /// 语音
    #[serde(rename = "record")]
    Record {
        /// 语音文件名
        file: String,
        /// 是否变声 1|0
        magic: Option<u8>,
        /// 语音 URL    
        url: Option<String>,
        /// 是否使用缓存文件 1|0
        cache: Option<u8>,
        /// 是否使用代理 1|0
        proxy: Option<u8>,
        /// 网络文件下载超时 单位秒
        timeout: Option<i64>,
    },

    /// 短视频
    #[serde(rename = "video")]
    Video {
        /// 视频地址
        file: String,
        /// 视频封面
        cover: String,
        /// 通过网络下载视频时的线程数 2|3
        c: Option<u8>,
    },

    /// @某人
    #[serde(rename = "at")]
    At {
        /// @QQ ID all 表示全体
        qq: String,
        /// 当在群中找不到此QQ号的名称时才会生效
        name: Option<String>,
    },

    /// 猜拳魔法表情
    #[serde(rename = "rps")]
    Rps,

    /// 掷骰子魔法表情
    #[serde(rename = "dice")]
    Dice,

    /// 窗口抖动（戳一戳）
    #[serde(rename = "shake")]
    Shake,

    /// 戳一戳
    #[serde(rename = "poke")]
    Poke {
        /// ID
        qq: String,
    },

    /// 匿名发消息
    #[serde(rename = "anonymous")]
    Anonymous,

    /// 链接分享
    #[serde(rename = "share")]
    Share {
        /// URL
        url: String,
        /// 标题
        title: String,
        /// 内容描述
        content: Option<String>,
        /// 图片 URl
        image: Option<String>,
    },

    /// 推荐好友|群
    #[serde(rename = "contact")]
    Contact {
        /// 类型 qq|group
        #[serde(rename = "type")]
        ty: String,
        /// QQ号|群号
        id: String,
    },

    /// 位置
    #[serde(rename = "location")]
    Lacation {
        /// 纬度
        lat: String,
        /// 经度           
        lon: String,
        /// 标题  
        title: Option<String>,
        /// 内容描述
        content: Option<String>,
    },

    /// 音乐分享
    #[serde(rename = "music")]
    Music {
        /// 类型 qq|163|xm|custom
        #[serde(rename = "type")]
        ty: String,
        /// 歌曲 ID
        id: Option<String>,
        /// 点击后跳转 URL
        url: Option<String>,
        /// 歌曲 URL  
        audio: Option<String>,
        /// 标题   
        title: Option<String>,
        /// 内容描述
        content: Option<String>,
        /// 图片 URl
        image: Option<String>,
    },

    /// 回复
    #[serde(rename = "reply")]
    Reply {
        /// 回复的消息 ID
        id: String,
        text: Option<String>,
        qq: Option<String>,
        time: Option<String>,
        seq: Option<String>,
    },

    /// 合并转发
    #[serde(rename = "forward")]
    Forward {
        /// 合并转发 ID
        id: String,
    },

    /// 合并转发节点
    #[serde(rename = "node")]
    Node {
        /// 转发的消息 ID
        id: Option<String>,
        /// 发送者 QQ 号        
        user_id: Option<String>,
        /// 发送者昵称   
        nickname: Option<String>,
        /// 消息内容     
        content: Option<MessageChain>,
    },

    /// XML 消息
    #[serde(rename = "xml")]
    Xml {
        /// 合并转发 ID
        data: String,
    },

    /// JSON 消息
    #[serde(rename = "json")]
    Json {
        /// 合并转发 ID
        data: String,
    },
}

macro_rules! message_builder {
    ($fn_name: ident, $message_type: tt) => {
        pub fn $fn_name() -> Message {
            Message::$message_type
        }
    };
    ($fn_name: ident, $message_type: tt, $param: ident: $param_ty: ty) => {
        pub fn $fn_name($param: $param_ty) -> Message {
            Message::$message_type { $param: $param }
        }
    };
    ($fn_name: ident, $message_type: tt, $($param: ident: $param_ty: ty),*) => {
        pub fn $fn_name($($param: $param_ty,)*) -> Message {
            Message::$message_type { $($param: $param,)* }
        }
    };
}

impl Message {
    pub fn text<T: AsRef<str>>(text: T) -> Message {
        Message::Text {
            text: text.as_ref().to_string(),
        }
    }
    message_builder!(face, Face, id: String);
    message_builder!(
        image,
        Image,
        file: String,
        ty: Option<String>,
        url: Option<String>,
        cache: Option<u8>,
        proxy: Option<u8>,
        timeout: Option<i64>
    );
    message_builder!(
        record,
        Record,
        file: String,
        magic: Option<u8>,
        url: Option<String>,
        cache: Option<u8>,
        proxy: Option<u8>,
        timeout: Option<i64>
    );
    message_builder!(
        video,
        Video,
        file: String,
        cover: String,
        c: Option<u8>
    );

    pub fn at(qq: String) -> Message {
        Message::At {
            qq,
            name: None,
        }
    }
    message_builder!(at_name, At, qq: String,name:Option<String>);
    message_builder!(rps, Rps);
    message_builder!(dice, Dice);
    message_builder!(shake, Shake);
    message_builder!(poke, Poke, qq: String);
    message_builder!(anonymous, Anonymous);
    message_builder!(
        share,
        Share,
        url: String,
        title: String,
        content: Option<String>,
        image: Option<String>
    );
    message_builder!(contact, Contact, ty: String, id: String);
    message_builder!(
        location,
        Lacation,
        lat: String,
        lon: String,
        title: Option<String>,
        content: Option<String>
    );
    message_builder!(
        music,
        Music,
        ty: String,
        id: Option<String>,
        url: Option<String>,
        audio: Option<String>,
        title: Option<String>,
        content: Option<String>,
        image: Option<String>
    );
    message_builder!(
        reply,
        Reply,
        id: String,
        text:Option<String>,
        qq:Option<String>,
        time:Option<String>,
        seq:Option<String>
    );
    message_builder!(forward, Forward, id: String);
    message_builder!(
        node,
        Node,
        id: Option<String>,
        user_id: Option<String>,
        nickname: Option<String>,
        content: Option<crate::message::MessageChain>
    );
    message_builder!(xml, Xml, data: String);
    message_builder!(json, Json, data: String);
}


pub struct MessageChainBuilder {
    inner: MessageChain,
}
macro_rules! message_chain_fn {
    ($fn_name: ident) => {
        pub fn $fn_name(&mut self,) -> &mut MessageChainBuilder {
            self.inner.push(Message::$fn_name());
            self
        }
    };
    ($fn_name: ident, $param: ident: $param_ty: ty) => {
        pub fn $fn_name(&mut self,$param: $param_ty) -> &mut MessageChainBuilder {
            self.inner.push(Message::$fn_name($param.to_string()));
            self
        }
    };
    ($fn_name: ident, $($param: ident: $param_ty: ty),*) => {
        pub fn $fn_name(&mut self,$($param: $param_ty),*) -> &mut MessageChainBuilder {
            self.inner.push(Message::$fn_name($($param.to_string()),*));
            self
        }
    };
}
pub type MessageChain = Vec<Message>;

impl MessageChainBuilder {
    pub fn new() -> MessageChainBuilder {
        MessageChainBuilder {
            inner: vec![],
        }
    }
    pub fn append(&mut self, message: Message) -> &mut MessageChainBuilder {
        self.inner.push(message);
        self
    }
    pub fn text<T: AsRef<str>>(&mut self, text: T) -> &mut MessageChainBuilder {
        self.inner.push(Message::text(text));
        self
    }
    message_chain_fn!(face, id: &str);


    pub fn at(&mut self, qq: i64) -> &mut MessageChainBuilder {
        self.inner.push(Message::at(qq.to_string()));
        self
    }
    pub fn image(&mut self, url: &str) -> &mut MessageChainBuilder {
        self.inner.push(Message::image(url.to_string(), None, None, None, None, None));
        self
    }

    message_chain_fn!(rps);
    message_chain_fn!(dice);
    message_chain_fn!(shake);

    pub fn poke(&mut self, qq: i64) -> &mut MessageChainBuilder {
        self.inner.push(Message::poke(qq.to_string()));
        self
    }
    pub fn reply(&mut self, message_id: i64) -> &mut MessageChainBuilder {
        self.inner.push(Message::reply(message_id.to_string(), None, None, None, None));
        self
    }
    pub fn reply_custom(&mut self, text: &str, qq: &str, time: i64, seq: i64) -> &mut MessageChainBuilder {
        self.inner.push(Message::reply("".to_string(), Some(text.to_string()), Some(qq.to_string()), Some(time.to_string()), Some(seq.to_string())));
        self
    }

    pub fn forward_node_custom(&mut self,
                               user_id: i64,
                               nickname: &str,
                               content: Vec<Message>) -> &mut MessageChainBuilder {
        self.inner.push(Message::node(None, Some(user_id.to_string()), Some(nickname.to_string()), Some(content)));
        self
    }

    message_chain_fn!(anonymous);
    message_chain_fn!(contact,ty: &str,id: &str);
    message_chain_fn!(forward, id: &str);

    message_chain_fn!(xml, data: &str);
    message_chain_fn!(json,  data: &str);

    pub fn build(&self) -> MessageChain {
        self.inner.clone()
    }
}