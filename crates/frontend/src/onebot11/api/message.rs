use serde::{Deserialize, Serialize};

use crate::onebot11::message::Message;

use super::endpoint::{ApiEndpoint, ApiParams};

/// 发送私聊消息
pub struct SendPrivateMsg;

impl ApiEndpoint for SendPrivateMsg {
    const ACTION_NAME: &'static str = "send_private_msg";
    type Params = SendPrivateMsgParam;
    type Response = SendPrivateMsgResponse;
}

/// | 字段名 | 数据类型 | 默认值 | 说明 |
/// | ----- | ------- | ----- | --- |
/// | `user_id` | number | - | 对方 QQ 号 |
/// | `message` | message | - | 要发送的内容 |
/// | `auto_escape` | boolean | `false` | 消息内容是否作为纯文本发送（即不解析 CQ 码），只在 `message` 字段是字符串时有效 |
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct SendPrivateMsgParam {
    pub user_id: i64,
    pub message: Vec<Message>,
    #[serde(default)]
    pub auto_escape: bool,
}

impl ApiParams for SendPrivateMsgParam {
    type Endpoint = SendPrivateMsg;
}

/// | 字段名 | 数据类型 | 说明 |
/// | ----- | ------- | --- |
/// | `message_id` | number (int32) | 消息 ID |
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct SendPrivateMsgResponse {
    pub message_id: i64,
}

/// 发送群消息
pub struct SendGroupMsg;

impl ApiEndpoint for SendGroupMsg {
    const ACTION_NAME: &'static str = "send_group_msg";
    type Params = SendGroupMsgParam;
    type Response = SendGroupMsgResponse;
}

/// | 字段名 | 数据类型 | 默认值 | 说明 |
/// | ----- | ------- | ----- | --- |
/// | `user_id` | number | - | 对方 QQ 号 |
/// | `message` | message | - | 要发送的内容 |
/// | `auto_escape` | boolean | `false` | 消息内容是否作为纯文本发送（即不解析 CQ 码），只在 `message` 字段是字符串时有效 |
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct SendGroupMsgParam {
    pub group_id: i64,
    pub message: Vec<Message>,
    #[serde(default)]
    pub auto_escape: bool,
}

impl ApiParams for SendGroupMsgParam {
    type Endpoint = SendGroupMsg;
}

/// | 字段名 | 数据类型 | 说明 |
/// | ----- | ------- | --- |
/// | `message_id` | number (int32) | 消息 ID |
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct SendGroupMsgResponse {
    pub message_id: i64,
}
