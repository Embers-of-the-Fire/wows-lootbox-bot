use std::fmt::Debug;

use serde::{de::DeserializeOwned, Serialize};

use super::request::Request;

pub trait ApiEndpoint {
    const ACTION_NAME: &'static str;
    type Params: ApiParams<Endpoint = Self> + Debug;
    type Response: DeserializeOwned;
}

pub type Echo = i64;

pub trait ApiParams: Serialize + Sized {
    type Endpoint: ApiEndpoint<Params = Self>;

    fn into_request(self, echo: Option<Echo>) -> Request<Self::Endpoint> {
        Request {
            action: Self::Endpoint::ACTION_NAME,
            params: self,
            echo,
        }
    }
}
