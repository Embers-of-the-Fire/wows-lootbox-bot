use serde::{Deserialize, Serialize};

use super::endpoint::Echo;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Response<R> {
    Failed {
        retcode: i32,
        echo: Option<Echo>,
    },
    Async {
        retcode: i32,
        echo: Option<Echo>,
    },
    Ok {
        retcode: i32,
        echo: Option<Echo>,
        data: R,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Ok,
    Async,
    Failed,
    #[serde(other)]
    #[default]
    Unknown,
}
