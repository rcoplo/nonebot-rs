use std::fmt::{Display, Formatter};
use futures_util::TryFutureExt;
use crate::message::MessageChain;


pub enum CqCode {
    Text(String),
    Face(Face),
    Record(Record),
    Video(Video),
    At(At),
    Rps,
    Dice,
    Shake,
    Anonymous,
    Share(Share),
    Contact(Contact),
    Location(Location),
    Music(Music),
    Image(Image),
    Reply(Reply),
    RedBag(RedBag),
    Poke(Poke),
    Gift(Gift),
    Forward(Forward),
    Node(Node),
    XmlMsg(XmlMsg),
    JsonMsg(JsonMsg),
}

/// 表情
#[derive(Default, Clone)]
pub struct Face {
    pub id: String,
}

impl Display for Face {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[CQ:face,id={}]", self.id)
    }
}

/// 语音
#[derive(Default, Clone)]
pub struct Record {
    pub file: String,
    pub magic: Option<u8>,
    pub url: Option<String>,
    pub cache: Option<u8>,
    pub proxy: Option<u8>,
    pub timeout: Option<i64>,
}

impl Display for Record {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[CQ:record,file={}{}{}{}{}{}]",
               self.file,
               &self.magic.map(|magic| format!(",magic={magic}")).unwrap_or_else(|| "".to_owned()),
               self.url.as_ref().map(|url| format!(",url={url}")).unwrap_or_else(|| "".to_owned()),
               self.cache.map(|cache| format!(",cache={cache}")).unwrap_or_else(|| "".to_owned()),
               self.proxy.map(|proxy| format!(",proxy={proxy}")).unwrap_or_else(|| "".to_owned()),
               self.timeout.map(|timeout| format!(",timeout={timeout}")).unwrap_or_else(|| "".to_owned()),
        )
    }
}

/// 视频
#[derive(Default, Clone)]
pub struct Video {
    pub file: String,
    /// 格式必须为jpg http|file|base64
    pub cover: String,
    pub c: Option<u8>,
}

impl Display for Video {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[CQ:video,file={},cover={}{}]",
               self.file,
               self.cover,
               self.c.map(|c| format!(",c={c}")).unwrap_or_else(|| "".to_owned()),
        )
    }
}

/// @
#[derive(Default, Clone)]
pub struct At {
    pub qq: String,
    pub name: Option<String>,
}

impl Display for At {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[CQ:at,qq={}{}]",
               self.qq,
               self.name.as_ref().map(|name| format!(",name={name}")).unwrap_or_else(|| "".to_owned()),
        )
    }
}

/// go-cqhttp 未支持
#[derive(Default, Clone)]
pub struct Rps {}

#[derive(Default, Clone)]
pub struct Dice {}

#[derive(Default, Clone)]
pub struct Shake {}

#[derive(Default, Clone)]
pub struct Anonymous {}

/// 链接分享
#[derive(Default, Clone)]
pub struct Share {
    pub url: String,
    pub title: String,
    pub content: Option<String>,
    pub image: Option<String>,
}

impl Display for Share {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[CQ:share,url={},title={}{}{}]",
               self.url,
               self.title,
               self.content.as_ref().map(|content| format!(",content={content}")).unwrap_or_else(|| "".to_owned()),
               self.image.as_ref().map(|image| format!(",image={image}")).unwrap_or_else(|| "".to_owned()),
        )
    }
}

/// 未支持 <br/>
/// 推荐好友/群
#[derive(Default, Clone)]
pub struct Contact {
    pub ty: String,
    pub id: i64,
}

/// 位置
#[derive(Default, Clone)]
pub struct Location {
    pub lat: f64,
    pub lon: f64,
    pub title: Option<String>,
    pub content: Option<String>,
}

/// 音乐分享 发
#[derive(Default, Clone)]
pub struct Music {
    pub ty: String,
    pub id: i64,
}

impl Display for Music {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[CQ:music,type={},id={}]",
               self.ty,
               self.id,
        )
    }
}

/// 音乐自定义分享 发
#[derive(Default, Clone)]
pub struct MusicCustom {
    pub id: i64,
    pub audio: String,
    pub title: String,
    pub content: Option<String>,
    pub image: Option<String>,
}

impl Display for MusicCustom {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[CQ:music,id={},audio={},title={}{}{}]",
               self.id,
               self.audio,
               self.title,
               self.content.as_ref().map(|content| format!(",content={content}")).unwrap_or_else(|| "".to_owned()),
               self.image.as_ref().map(|image| format!(",image={image}")).unwrap_or_else(|| "".to_owned()),
        )
    }
}

/// 图片 收/发
#[derive(Default, Clone)]
pub struct Image {
    pub file: String,
    pub ty: Option<String>,
    pub sub_type: Option<u8>,
    pub url: Option<String>,
    pub cache: Option<u8>,
    pub id: Option<i32>,
    pub c: Option<u8>,
}

impl Display for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[CQ:image,file={}{}{}{}{}{}]",
               self.file,
               self.ty.as_ref().map(|ty| format!(",type={ty}")).unwrap_or_else(|| "".to_owned()),
               self.sub_type.map(|sub_type| format!(",sub_type={sub_type}")).unwrap_or_else(|| "".to_owned()),
               self.url.as_ref().map(|url| format!(",url={url}")).unwrap_or_else(|| "".to_owned()),
               self.id.map(|id| format!(",id={id}")).unwrap_or_else(|| "".to_owned()),
               self.c.map(|c| format!(",c={c}")).unwrap_or_else(|| "".to_owned()),
        )
    }
}

/// 回复 收/发
#[derive(Default, Clone)]
pub struct Reply {
    pub id: i32,
    pub text: Option<String>,
    pub qq: Option<i64>,
    pub time: Option<i64>,
    pub seq: Option<i64>,
}

impl Display for Reply {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[CQ:reply,id={}{}{}{}{}]",
               self.id,
               self.text.as_ref().map(|text| format!(",text={text}")).unwrap_or_else(|| "".to_owned()),
               self.qq.map(|qq| format!(",qq={qq}")).unwrap_or_else(|| "".to_owned()),
               self.time.map(|time| format!(",time={time}")).unwrap_or_else(|| "".to_owned()),
               self.seq.map(|seq| format!(",seq={seq}")).unwrap_or_else(|| "".to_owned()),
        )
    }
}

/// 红包 收
#[derive(Default, Clone)]
pub struct RedBag {
    pub title: String,
}

/// 戳一戳 发
#[derive(Default, Clone)]
pub struct Poke {
    pub qq: i64,
}

impl Display for Poke {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[CQ:poke,qq={}]",
               self.qq,
        )
    }
}

/// 礼物 发
#[derive(Default, Clone)]
pub struct Gift {
    pub qq: i64,
    pub id: u8,
}

impl Display for Gift {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[CQ:gift,qq={},id={}]",
               self.qq,
               self.id,
        )
    }
}

/// 合并转发 收
#[derive(Default, Clone)]
pub struct Forward {
    pub id: String,
}

/// 合并转发消息节点
#[derive(Default, Clone)]
pub struct Node {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub uin: Option<i64>,
    pub content: Option<String>,
    pub seq: Option<MessageChain>,
}

/// xml消息 收/发
#[derive(Default, Clone)]
pub struct XmlMsg {
    pub data: String,

}

impl Display for XmlMsg {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[CQ:xml,data={}]",
               self.data,
        )
    }
}

/// json消息 收/发
#[derive(Default, Clone)]
pub struct JsonMsg {
    pub data: String,
}

impl Display for JsonMsg {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[CQ:json,data={}]",
               self.data.replace(",", "&#44;")
                   .replace("&", "&amp;")
                   .replace("[", "&#91;")
                   .replace("]", "&#93;"),
        )
    }
}

/// 装逼大图 发
#[derive(Default, Clone)]
pub struct CardImage {
    pub file: String,
    pub minwidth: Option<i64>,
    pub minheight: Option<i64>,
    pub maxwidth: Option<i64>,
    pub maxheight: Option<i64>,
    pub source: Option<String>,
    pub icon: Option<String>,
}

impl Display for CardImage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[CQ:cardimage,file={}{}{}{}{}{}{}]",
               self.file,
               self.minwidth.map(|minwidth| format!(",minwidth={minwidth}")).unwrap_or_else(|| "".to_owned()),
               self.minheight.map(|minheight| format!(",minheight={minheight}")).unwrap_or_else(|| "".to_owned()),
               self.maxwidth.map(|maxwidth| format!(",maxwidth={maxwidth}")).unwrap_or_else(|| "".to_owned()),
               self.maxheight.map(|maxheight| format!(",maxheight={maxheight}")).unwrap_or_else(|| "".to_owned()),
               self.source.as_ref().map(|source| format!(",source={source}")).unwrap_or_else(|| "".to_owned()),
               self.icon.as_ref().map(|icon| format!(",icon={icon}")).unwrap_or_else(|| "".to_owned()),
        )
    }
}

/// 文本转语音 发
#[derive(Default, Clone)]
pub struct Ttc {
    pub text: String,
}

impl Display for Ttc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[CQ:ttc,text={}]",
               self.text,
        )
    }
}











