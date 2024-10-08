use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Hash)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Message {
    Text {
        data: Text,
    },
    Face {
        data: Face,
    },
    Image {
        data: Image,
    },
    Reply {
        data: Reply,
    },
    Forward {
        data: ForwardMessage,
    },
    Node {
        data: ForwardMessageNode,
    },
    #[serde(other)]
    Verbatim,
}

impl Message {
    pub fn stringify(&self) -> &str {
        match self {
            Self::Text { data } => &data.text,
            _ => "",
        }
    }
}

macro_rules! __impl_msg {
    ($($ident:ident $(as $tag:ident)?),+ $(,)?) => {
        $(__impl_msg!(@$ident $(as $tag)?);)+
    };
    (@$ident:ident) => {
        impl Into<Message> for $ident {
            fn into(self) -> Message {
                Message::$ident { data: self }
            }
        }

        impl TryFrom<Message> for $ident {
            type Error = ();
            fn try_from(msg: Message) -> Result<Self, ()> {
                match msg {
                    Message::$ident { data } => Ok(data),
                    _ => Err(())
                }
            }
        }
    };
    (@$ident:ident as $tag:ident) => {
        impl Into<Message> for $ident {
            fn into(self) -> Message {
                Message::$tag { data: self }
            }
        }

        impl TryFrom<Message> for $ident {
            type Error = ();
            fn try_from(msg: Message) -> Result<Self, ()> {
                match msg {
                    Message::$tag { data } => Ok(data),
                    _ => Err(())
                }
            }
        }
    };
}

__impl_msg!(
    Text,
    Face,
    Image,
    Reply,
    ForwardMessage as Forward,
    ForwardMessageNode as Node
);

/// 纯文本
///
/// | 参数名| 收 | 发 | 可能的值 | 说明 |
/// | --- | --- | --- | --- | --- |
/// | `text` | ✓ | ✓ | - | 纯文本内容 |
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub struct Text {
    pub text: String,
}

/// QQ 表情
///
/// | 参数名 | 收 | 发 | 可能的值 | 说明 |
/// | --- | --- | --- | --- | --- |
/// | `id` | ✓ | ✓ | 见 [QQ 表情 ID 表](https://github.com/richardchien/coolq-http-api/wiki/%E8%A1%A8%E6%83%85-CQ-%E7%A0%81-ID-%E8%A1%A8) | QQ 表情 ID |
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub struct Face {
    #[serde(
        serialize_with = "utils::serde_int_str::serialize_to_str",
        deserialize_with = "utils::serde_int_str::deserialize_from_str"
    )]
    pub id: u32,
}

/// 图片
///
/// | 参数名 | 收 | 发 | 可能的值 | 说明 |
/// | --- | --- | --- | --- | --- |
/// | `file` | ✓ | ✓<sup>[1]</sup> | - | 图片文件名 |
/// | `type` | ✓ | ✓ | `flash` | 图片类型，`flash` 表示闪照，无此参数表示普通图片 |
/// | `url` | ✓ |  | - | 图片 URL |
/// | `cache` |  | ✓ | `0` `1` | 只在通过网络 URL 发送时有效，表示是否使用已缓存的文件，默认 `1` |
/// | `proxy` |  | ✓ | `0` `1` | 只在通过网络 URL 发送时有效，表示是否通过代理下载文件（需通过环境变量或配置文件配置代理），默认 `1` |
/// | `timeout` |  | ✓ | - | 只在通过网络 URL 发送时有效，单位秒，表示下载网络文件的超时时间，默认不超时 |
///
/// [1] 发送时，`file` 参数除了支持使用收到的图片文件名直接发送外，还支持：
///
/// - 绝对路径，例如 `file:///C:\\Users\Richard\Pictures\1.png`，格式使用 [`file` URI](https://tools.ietf.org/html/rfc8089)
/// - 网络 URL，例如 `http://i1.piimg.com/567571/fdd6e7b6d93f1ef0.jpg`
/// - Base64 编码，例如 `base64://iVBORw0KGgoAAAANSUhEUgAAABQAAAAVCAIAAADJt1n/AAAAKElEQVQ4EWPk5+RmIBcwkasRpG9UM4mhNxpgowFGMARGEwnBIEJVAAAdBgBNAZf+QAAAAABJRU5ErkJggg==`
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Hash, Default)]
pub struct Image {
    pub file: String,
    #[serde(rename = "type", default)]
    pub image_type: ImageType,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(
        default = "utils::primitive_default::bool_true",
        deserialize_with = "super::serde_utils::deserialize_onebot_bool"
    )]
    pub cache: bool,
    #[serde(
        default = "utils::primitive_default::bool_true",
        deserialize_with = "super::serde_utils::deserialize_onebot_bool"
    )]
    pub proxy: bool,
    #[serde(default)]
    pub timeout: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Default, Hash)]
pub enum ImageType {
    Flash,
    #[serde(other)]
    #[default]
    Normal,
}

impl Serialize for ImageType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Flash => serializer.serialize_str("flash"),
            Self::Normal => serializer.serialize_none(),
        }
    }
}

/// 回复
///
/// | 参数名 | 收 | 发 | 可能的值 | 说明 |
/// | --- | --- | --- | --- | --- |
/// | `id` | ✓ | ✓ | - | 回复时引用的消息 ID |
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub struct Reply {
    #[serde(
        serialize_with = "utils::serde_int_str::serialize_to_str",
        deserialize_with = "utils::serde_int_str::deserialize_from_str"
    )]
    pub id: i32,
}

/// 合并转发（**仅接收**）
///
/// | 参数名 | 收 | 发 | 可能的值 | 说明 |
/// | --- | --- | --- | --- | --- |
/// | `id` | ✓ |  | - | 合并转发 ID，需通过 [`get_forward_msg` API](https://github.com/botuniverse/onebot-11/blob/master/api/public.md#get_forward_msg-获取合并转发消息) 获取具体内容 |
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub struct ForwardMessage {
    #[serde(
        serialize_with = "utils::serde_int_str::serialize_to_str",
        deserialize_with = "utils::serde_int_str::deserialize_from_str"
    )]
    pub id: i32,
}

/// 合并转发节点
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(untagged)]
pub enum ForwardMessageNode {
    /// 合并转发节点（**仅发送**）
    ///
    /// | 参数名 | 收 | 发 | 可能的值 | 说明 |
    /// | --- | --- | --- | --- | --- |
    /// | `id` |  | ✓ | - | 转发的消息 ID |
    Node {
        #[serde(
            serialize_with = "utils::serde_int_str::serialize_to_str",
            deserialize_with = "utils::serde_int_str::deserialize_from_str"
        )]
        id: i32,
    },
    /// 合并转发自定义节点
    ///
    /// > **注意**
    /// >
    /// > 接收时，此消息段不会直接出现在消息事件的 `message` 中，
    /// 需通过 [`get_forward_msg` API](../api/public.md#get_forward_msg-获取合并转发消息) 获取。
    ///
    /// | 参数名 | 收 | 发 | 可能的值 | 说明 |
    /// | --- | --- | --- | --- | --- |
    /// | `user_id` | ✓ | ✓ | - | 发送者 QQ 号 |
    /// | `nickname` | ✓ | ✓ | - | 发送者昵称 |
    /// | `content` | ✓ | ✓ | - | 消息内容 |
    Custom {
        #[serde(
            serialize_with = "utils::serde_int_str::serialize_to_str",
            deserialize_with = "utils::serde_int_str::deserialize_from_str"
        )]
        user_id: u32,
        nickname: String,
        content: Vec<Message>,
    },
}

pub fn stringify(msg: &[Message]) -> String {
    msg.iter().map(Message::stringify).join(" ")
}

#[test]
fn test_parse_message() -> Result<(), Box<dyn std::error::Error>> {
    let raw = serde_json::json! {[
        {
            "type": "text",
            "data": {
                "text": "纯文本内容"
            }
        },
        {
            "type": "image",
            "data": {
                "file": "http://baidu.com/1.jpg",
                "cache": "no"
            }
        },
        {
            "type": "face",

            "data": {
                "id": "123"
            }
        },
        {
            "type": "node",
            "data": {
                "user_id": "10001000",
                "nickname": "某人",
                "content": [
                    {"type": "face", "data": {"id": "123"}},
                    {"type": "text", "data": {"text": "哈喽～"}}
                ]
            }
        }
    ]};
    let parsed: Vec<Message> = serde_json::from_value(raw)?;
    println!("{:#?}", parsed);

    Ok(())
}
