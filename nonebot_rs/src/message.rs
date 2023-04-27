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
        qq: i64,
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
        id: i64,
    },

    /// 位置
    #[serde(rename = "location")]
    Lacation {
        /// 纬度
        lat: f64,
        /// 经度           
        lon: f64,
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
        id: Option<i64>,
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
        id: i32,
        text: Option<String>,
        qq: Option<i64>,
        time: Option<i64>,
        seq: Option<i64>,
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
        id: Option<i32>,
        /// 发送者 QQ 号        
        uin: Option<i64>,
        /// 发送者昵称   
        name: Option<String>,
        /// 消息内容
        content: Option<String>,
        /// 消息内容     
        seq: Option<MessageChain>,

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

pub type MessageChain = Vec<Message>;
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
    message_builder!(poke, Poke, qq: i64);
    message_builder!(anonymous, Anonymous);
    message_builder!(
        share,
        Share,
        url: String,
        title: String,
        content: Option<String>,
        image: Option<String>
    );
    message_builder!(contact, Contact, ty: String, id: i64);
    message_builder!(
        location,
        Lacation,
        lat: f64,
        lon: f64,
        title: Option<String>,
        content: Option<String>
    );
    message_builder!(
        music,
        Music,
        ty: String,
        id: Option<i64>,
        url: Option<String>,
        audio: Option<String>,
        title: Option<String>,
        content: Option<String>,
        image: Option<String>
    );
    message_builder!(
        reply,
        Reply,
        id: i32,
        text:Option<String>,
        qq:Option<i64>,
        time:Option<i64>,
        seq:Option<i64>
    );
    message_builder!(forward, Forward, id: String);
    message_builder!(
        node,
        Node,
         id: Option<i32>,
        uin: Option<i64>,
        name: Option<String>,
        content: Option<String>,
        seq: Option<MessageChain>
    );
    message_builder!(xml, Xml, data: String);
    message_builder!(json, Json, data: String);
}

pub trait MessageChainTrait {
    fn new() -> MessageChain;
    fn append(&mut self, message: Message) -> &mut MessageChain;
    fn text<T: AsRef<str>>(&mut self, text: T) -> &mut MessageChain;
    fn at(&mut self, qq: i64) -> &mut MessageChain;
    fn image(&mut self, url: &str) -> &mut MessageChain;
    fn image_custom(&mut self, file: &str, ty: Option<&str>, url: Option<&str>, cache: Option<u8>, proxy: Option<u8>, timeout: Option<i64>) -> &mut MessageChain;
    fn poke(&mut self, qq: i64) -> &mut MessageChain;
    fn reply(&mut self, message_id: i32) -> &mut MessageChain;
    fn reply_custom(&mut self, text: &str, qq: i64, time: i64, seq: i64) -> &mut MessageChain;
    fn forward_node_custom(&mut self, user_id: i64, nickname: &str, content: Vec<Message>) -> &mut MessageChain;
    fn music(&mut self, ty: &str, id: i64) -> &mut MessageChain;
    fn music_custom(&mut self, ty: &str, id: Option<i64>, url: Option<&str>, audio: Option<&str>, title: Option<&str>, content: Option<&str>, image: Option<&str>) -> &mut MessageChain;
    fn face(&mut self, id: &str) -> &mut MessageChain;
    fn rps(&mut self) -> &mut MessageChain;
    fn dice(&mut self) -> &mut MessageChain;
    fn shake(&mut self) -> &mut MessageChain;
    fn anonymous(&mut self) -> &mut MessageChain;
    fn contact(&mut self, ty: &str, id: i64) -> &mut MessageChain;
    fn forward(&mut self, id: &str) -> &mut MessageChain;
    fn xml(&mut self, data: &str) -> &mut MessageChain;
    fn json(&mut self, data: &str) -> &mut MessageChain;
    fn build(&self) -> MessageChain;
}

impl MessageChainTrait for MessageChain {
    fn new() -> MessageChain {
        vec![]
    }

    fn append(&mut self, message: Message) -> &mut MessageChain {
        self.push(message);
        self
    }

    fn text<T: AsRef<str>>(&mut self, text: T) -> &mut MessageChain {
        self.push(Message::text(text));
        self
    }

    fn at(&mut self, qq: i64) -> &mut MessageChain {
        self.push(Message::at(qq.to_string()));
        self
    }

    fn image(&mut self, url: &str) -> &mut MessageChain {
        self.push(Message::image(url.to_string(), None, None, None, None, None));
        self
    }

    fn image_custom(&mut self, file: &str, ty: Option<&str>, url: Option<&str>, cache: Option<u8>, proxy: Option<u8>, timeout: Option<i64>) -> &mut MessageChain {
        self.push(Message::image(file.to_string(), ty.map(|ty| { ty.to_string() }), url.map(|url| { url.to_string() }), cache, proxy, timeout));
        self
    }

    fn poke(&mut self, qq: i64) -> &mut MessageChain {
        self.push(Message::poke(qq));
        self
    }

    fn reply(&mut self, message_id: i32) -> &mut MessageChain {
        self.push(Message::reply(message_id, None, None, None, None));
        self
    }

    fn reply_custom(&mut self, text: &str, qq: i64, time: i64, seq: i64) -> &mut MessageChain {
        self.push(Message::reply(0, Some(text.to_string()), Some(qq), Some(time), Some(seq)));
        self
    }

    fn forward_node_custom(&mut self, user_id: i64, nickname: &str, content: Vec<Message>) -> &mut MessageChain {
        self.push(Message::node(None, Some(user_id), Some(nickname.to_string()), None, Some(content)));
        self
    }

    fn music(&mut self, ty: &str, id: i64) -> &mut MessageChain {
        self.push(Message::music(ty.to_string(), Some(id), None, None, None, None, None));
        self
    }

    fn music_custom(&mut self, ty: &str, id: Option<i64>, url: Option<&str>, audio: Option<&str>, title: Option<&str>, content: Option<&str>, image: Option<&str>) -> &mut MessageChain {
        self.push(Message::music(ty.to_string(), id,
                                 url.map(|url| { url.to_string() }),
                                 audio.map(|audio| { audio.to_string() }),
                                 title.map(|title| { title.to_string() }),
                                 content.map(|content| { content.to_string() }),
                                 image.map(|image| { image.to_string() }),
        ));
        self
    }

    fn face(&mut self, id: &str) -> &mut MessageChain {
        self.push(Message::face(id.to_string()));
        self
    }

    fn rps(&mut self) -> &mut MessageChain {
        self.push(Message::rps());
        self
    }

    fn dice(&mut self) -> &mut MessageChain {
        self.push(Message::dice());
        self
    }

    fn shake(&mut self) -> &mut MessageChain {
        self.push(Message::shake());
        self
    }

    fn anonymous(&mut self) -> &mut MessageChain {
        self.push(Message::anonymous());
        self
    }

    fn contact(&mut self, ty: &str, id: i64) -> &mut MessageChain {
        self.push(Message::contact(ty.to_string(), id));
        self
    }

    fn forward(&mut self, id: &str) -> &mut MessageChain {
        self.push(Message::forward(id.to_string()));
        self
    }

    fn xml(&mut self, data: &str) -> &mut MessageChain {
        self.push(Message::xml(data.to_string()));
        self
    }

    fn json(&mut self, data: &str) -> &mut MessageChain {
        self.push(Message::json(data.to_string()));
        self
    }

    fn build(&self) -> MessageChain {
        self.clone()
    }
}

