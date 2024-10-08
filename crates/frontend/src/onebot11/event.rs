use serde::{Deserialize, Serialize};

use super::message::Message;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Event {
    pub time: i64,
    pub self_id: i64,
    #[serde(flatten)]
    pub content: EventContent,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(tag = "post_type", rename_all = "snake_case")]
pub enum EventContent {
    Message(MessageEvent),
    MetaEvent(MetaEvent),
    #[serde(other)]
    #[default]
    Verbatim,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "message_type", rename_all = "snake_case")]
pub enum MessageEvent {
    Private(PrivateMessage),
    Group(GroupMessage),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PrivateMessage {
    #[serde(default)]
    pub sub_type: PrivateMessageType,
    pub message_id: i32,
    pub user_id: i64,
    pub message: Vec<Message>,
    pub raw_message: String,
    pub font: i32,
    #[serde(default)]
    pub sender: PrivateMessageSender,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum PrivateMessageType {
    Frient,
    Group,
    #[serde(other)]
    #[default]
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct PrivateMessageSender {
    #[serde(default)]
    pub user_id: Option<i64>,
    #[serde(default)]
    pub nickname: Option<String>,
    #[serde(default)]
    pub sex: Option<SexType>,
    #[serde(default)]
    pub age: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GroupMessage {
    #[serde(default)]
    pub sub_type: GroupMessageType,
    pub message_id: i32,
    pub group_id: i64,
    pub user_id: i64,
    pub message: Vec<Message>,
    pub raw_message: String,
    pub font: i32,
    #[serde(default)]
    pub anonymous: Option<AnonymousSender>,
    #[serde(default)]
    pub sender: GroupMessageSender,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct GroupMessageSender {
    pub user_id: Option<i64>,
    pub nickname: Option<String>,
    pub card: Option<String>,
    pub sex: Option<SexType>,
    pub age: Option<i32>,
    pub area: Option<String>,
    pub level: Option<String>,
    pub role: Option<GroupRole>,
    pub title: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AnonymousSender {
    pub id: i64,
    pub name: String,
    pub flag: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum GroupMessageType {
    Anonymous,
    Notice,
    #[serde(other)]
    #[default]
    Normal,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SexType {
    Male,
    Female,
    #[serde(other)]
    #[default]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum GroupRole {
    Owner,
    Admin,
    #[serde(other)]
    #[default]
    Member,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "meta_event_type", rename_all = "snake_case")]
pub enum MetaEvent {
    Lifecycle(Lifecycle),
    Heartbeat(Heartbeat),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Lifecycle {
    pub sub_type: LifecycleSubtype,
}

/// **注意**，目前生命周期元事件中，只有 HTTP POST 的情况下可以收到 `enable` 和 `disable`，
/// 只有正向 WebSocket 和反向 WebSocket 可以收到 `connect`。
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleSubtype {
    Enable,
    Disable,
    Connect,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Heartbeat {
    #[serde(default)]
    pub status: serde_json::Map<String, serde_json::Value>,
    pub interval: i64,
}

#[test]
fn test_parse_event() -> Result<(), Box<dyn std::error::Error>> {
    let raw = serde_json::json! {{
        "time": 1599999999,
        "self_id": 123456789,
        "post_type": "message",
        "message_type": "private",
        "sub_type": "group",
        "message_id": 123456789,
        "user_id": 123456789,
        "message": [{
            "type": "text",
            "data": {
                "text": "hello"
            }
        }],
        "raw_message": "hello",
        "font": 0
    }};

    let parsed: Event = serde_json::from_value(raw)?;
    println!("{:#?}", parsed);

    Ok(())
}
