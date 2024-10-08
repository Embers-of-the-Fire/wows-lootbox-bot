use std::{cmp::Reverse, collections::HashMap};

use axum::Json;
use bson::doc;
use itertools::Itertools;
use log::{debug, info};
use mongodb::{Client, Collection};
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use strsim::sorensen_dice;
use utils::{tri, triany};
use wows_box::lootbox::LootBox;
use wows_box_render::process::render_to_file;

use crate::{AppResponse, AppState, APP_STATE};

// const USAGE: &str = r#"使用方法：
// box <物品名称> <数量>
// 示例：
// box 超级补给箱 100
// 备注：
// 有多个相似名称时会出错"#;
const NO_ITEM_FOUND: &str = r#"未找到对应物品。"#;
// const INT_ERROR: &str = r#"数字输入错误"#;
const MULTIPLE_ITEM_FOUND: &str = r#"找到过多匹配项：\n"#;
const UNKNOWN_ERROR: &str = r#"机器人出错了！"#;

pub async fn rand_handler(Json(param): Json<BoxParam>) -> Json<AppResponse<Vec<Message>>> {
    info!("Connected with client.");

    debug!("Received: {:?}", param);

    let resp = handle_req(param, APP_STATE.get().await).await;

    println!("End connection.");

    Json(resp.into())
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Message {
    Text(String),
    Image(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoxParam {
    pub lang: String,
    pub box_name: String,
    pub amount: u32,
}

pub async fn handle_req(param: BoxParam, state: &AppState) -> anyhow::Result<Vec<Message>> {
    debug!("Receive request: {:?}", param);

    handle(param, &state.conn).await
}

#[tokio::test]
async fn test_msg() {
    use std::env;

    use dotenvy::dotenv;
    use mongodb::options::{ClientOptions, ServerApi, ServerApiVersion};

    dotenv().unwrap();

    let p = BoxParam {
        lang: "en".to_owned(),
        box_name: "Mini No.5".to_owned(),
        amount: 250,
    };

    let mut client_options = ClientOptions::parse(env::var("MONGODB_CONN").unwrap())
        .await
        .unwrap();
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);
    let client = Client::with_options(client_options).unwrap();

    let h = handle(p, &client).await;
    println!("{:#?}", h);
}

async fn handle(param: BoxParam, client: &Client) -> anyhow::Result<Vec<Message>> {
    let db = client.database(&format!("wowslootbox-{}", param.lang));
    let box_list: Collection<LootBox> = db.collection("list");
    let resp = box_list.find(doc! {}).allow_disk_use(true).await;

    let mut resp = triany!(warn resp);
    let mut map = HashMap::new();
    let mut keys = Vec::new();
    while triany!(warn resp.advance().await) {
        let next = resp.deserialize_current();
        let next = tri!(continue; warn next);
        keys.push((
            next.name.clone(),
            sorensen_dice(&next.name, &param.box_name),
        ));
        map.insert(next.name, next.id);
    }
    keys.sort_unstable_by_key(|&(_, n)| Reverse(OrderedFloat(n)));

    if let Some((first_key, first_rate)) = keys.first() {
        if (first_rate - 1.0) < 0.01 {
            let id = map.get(first_key).unwrap();
            Ok(build_img(&param.lang, client, *id, param.amount).await)
        } else {
            let filtered = keys.iter().filter(|(_, n)| *n > 0.5).collect_vec();
            if filtered.get(1).is_some() {
                Ok(vec![
                    Message::Text(MULTIPLE_ITEM_FOUND.to_owned()),
                    Message::Text(filtered.into_iter().map(|(s, _)| s).join("\n")),
                ])
            } else {
                let id = map.get(first_key).unwrap();
                debug!("Select lootbox {}", first_key);
                Ok(build_img(&param.lang, client, *id, param.amount).await)
            }
        }
    } else {
        Ok(vec![Message::Text(NO_ITEM_FOUND.to_owned())])
    }
}

async fn build_img(lang: &str, client: &Client, key: u64, times: u32) -> Vec<Message> {
    let path = render_to_file(lang, client, key, times).await;
    let path = tri!(return vec![Message::Text(UNKNOWN_ERROR.to_owned())]; warn path);
    vec![Message::Image(path)]
}
