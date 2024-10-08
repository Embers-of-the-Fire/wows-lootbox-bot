use std::time::Instant;

use anyhow::anyhow;
use bson::doc;
use log::{debug, info};
use mongodb::Client;

pub async fn update_boxlist(lang: &str, client: &Client) -> anyhow::Result<()> {
    info!("Started updating lootbox list [lang {}]...", lang);
    let time_c = Instant::now();

    let box_db = client.database(&format!("wowslootbox-{lang}"));
    let box_list_collection = box_db.collection("list");

    debug!("Started fetching box list...");
    let time = Instant::now();
    let box_list = wows_box_fetch::list::fetch_list(lang).await?;
    debug!("Fetched box list in {:.2}s", time.elapsed().as_secs_f64());

    for item in box_list {
        let id = item.id;
        debug!("Fetching item: {}", id);
        let res = box_list_collection
            .find_one(doc! { "id": id as u32 })
            .await?;
        if res.is_some() {
            debug!("Duplicate item: {}", id);
            continue;
        }
        let time = Instant::now();
        let box_detail = wows_box_fetch::lootbox::fetch_lootbox(lang, id)
            .await?
            .ok()
            .ok_or(anyhow!("Unable to fetch box detail"))?;
        debug!("Fetched box detail in {:.2}s", time.elapsed().as_secs_f64());
        box_list_collection
            .insert_one(box_detail.into_standrad())
            .await?;
    }

    info!("Updated box list in {:.2}s", time_c.elapsed().as_secs_f64());

    Ok(())
}
