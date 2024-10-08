use std::cmp::Reverse;

use axum::{extract::Query, Json};
use bson::doc;
use log::{debug, info};
use mongodb::Collection;
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use strsim::sorensen_dice;
use utils::{tri, triany};
use wows_box::lootbox::LootBox;

use crate::{AppResponse, APP_STATE};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchItem {
    name: String,
    score: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchQueryArg {
    pat: String,
    lang: String,
    limit: Option<u32>,
}

pub async fn search_handler(Query(q): Query<SearchQueryArg>) -> Json<AppResponse<Vec<SearchItem>>> {
    info!("Connected with client.");

    debug!("Received: {:?}", q);

    let data = handler(&q.pat, &q.lang, q.limit.unwrap_or(10)).await;

    println!("End connection.");

    Json(data.into())
}

async fn handler(pat: &str, lang: &str, lim: u32) -> anyhow::Result<Vec<SearchItem>> {
    let client = &APP_STATE.get().await.conn;
    let db = client.database(&format!("wowslootbox-{lang}"));
    let box_list: Collection<LootBox> = db.collection("list");
    let resp = box_list.find(doc! {}).allow_disk_use(true).await;
    let mut resp = triany!(warn resp);
    let mut items = Vec::new();
    while triany!(warn resp.advance().await) {
        let next = resp.deserialize_current();
        let next = tri!(continue; warn next);
        items.push(SearchItem {
            name: next.name.clone(),
            score: sorensen_dice(&next.name, &pat),
        });
    }

    items.sort_unstable_by_key(|t| Reverse(OrderedFloat(t.score)));

    if items.len() > lim as usize {
        items.drain((lim as usize + 1)..);
    }

    Ok(items)
}
