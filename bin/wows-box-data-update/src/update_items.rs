use std::time::Instant;

use bson::doc;
use log::{debug, info};
use mongodb::{Client, Collection};
use serde::{Deserialize, Serialize};

use wows_box::{
    item::ItemData,
    lootbox::{LootBox, LootBoxReward},
};
use wows_box_fetch::{album::fetch_album, item::fetch_item};

pub async fn update_items(lang: &str, client: &Client) -> anyhow::Result<()> {
    info!("Started updating item static data [lang {}]...", lang);
    let time_c = Instant::now();

    let box_db = client.database(&format!("wowslootbox-{lang}"));
    let lootbox_collection: Collection<LootBox> = box_db.collection("list");
    let items_collection: Collection<ItemData> = box_db.collection("items");

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct ConcatResponse {
        rewards: Vec<Vec<Vec<LootBoxReward>>>,
    }

    debug!("Started processing box list...");
    let time = Instant::now();
    let mut rewards = lootbox_collection
        .aggregate(vec![doc! {
            "$project": {
                "rewards": {
                    "$concatArrays": [
                        "$slots.common.rewards",
                        "$slots.valuable.rewards",
                    ]
                }
            }
        }])
        .allow_disk_use(true)
        .await?
        .with_type::<ConcatResponse>();
    debug!("Fetched box list in {:.2}s", time.elapsed().as_secs_f64());

    use wows_box::lootbox::LootBoxRewardType::*;

    while rewards.advance().await? {
        let curr: ConcatResponse = rewards.deserialize_current()?;
        for reward in curr.rewards.into_iter().flatten().flatten() {
            match reward.reward {
                CamoBoost { id } | Signal { id, .. } => {
                    debug!("Fetch reward item {}", id);
                    let res = items_collection.find_one(doc! { "id": id as u32 }).await?;
                    if res.is_some() {
                        debug!("Duplicate item: {}", id);
                        continue;
                    }
                    let time = Instant::now();
                    let fetched = fetch_item(lang, id).await?.into_standard();
                    debug!(
                        "Fetched item detail in {:.2}s",
                        time.elapsed().as_secs_f64()
                    );
                    items_collection.insert_one(fetched).await?;
                }
                CollectionAlbum { id } => {
                    debug!("Fetch reward item(album) {}", id);
                    let res = items_collection.find_one(doc! { "id": id as u32 }).await?;
                    if res.is_some() {
                        debug!("Duplicate item(album): {}", id);
                        continue;
                    }
                    let time = Instant::now();
                    let fetched = fetch_album(lang, id).await?.into_standard().into_item();
                    debug!(
                        "Fetched item(album) detail in {:.2}s",
                        time.elapsed().as_secs_f64()
                    );
                    items_collection.insert_one(fetched).await?;
                }
                _ => {}
            }
        }
    }

    info!(
        "Updated item data in {:.2}s",
        time_c.elapsed().as_secs_f64()
    );

    Ok(())
}
