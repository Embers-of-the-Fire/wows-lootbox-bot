use log::info;
use serde::{Deserialize, Serialize};

use super::{
    endpoint::{ApiEndpoint, Echo},
    response::Response,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Request<R: ApiEndpoint> {
    pub action: &'static str,
    pub params: R::Params,
    pub echo: Option<Echo>,
}

impl<R: ApiEndpoint> Request<R> {
    pub async fn send(
        self,
        root_url: impl AsRef<str>,
        // auth_key: impl AsRef<str>,
    ) -> anyhow::Result<Response<R::Response>> {
        let r_url = root_url.as_ref();
        info!(
            "Send request to `{}/{}`: {:?}",
            r_url, self.action, self.params
        );
        let response = reqwest::ClientBuilder::new()
            .build()?
            .post(format!("{}/{}", r_url, self.action))
            .json(&self.params)
            // .header("Authorization", auth_key.as_ref())
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }
}
