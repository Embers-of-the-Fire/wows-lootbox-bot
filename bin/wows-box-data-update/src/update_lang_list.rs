use std::time::Instant;

use bson::doc;
use log::info;
use mongodb::{Client, Collection};
use serde::Serialize;

pub async fn update_lang_list(langs: &[&str], client: &Client) -> anyhow::Result<()> {
    info!("Syncing language list...");
    let time_c = Instant::now();

    let db = client.database("wowslootbox-meta");
    let col: Collection<Lang> = db.collection("languages");
    col.delete_many(doc! {}).await?;

    #[derive(Debug, Serialize)]
    struct Lang<'a> {
        langs: &'a [&'a str],
    }

    let lang = Lang { langs };
    col.insert_one(lang).await?;

    info!(
        "Updated language list in {:.2}s",
        time_c.elapsed().as_secs_f64()
    );

    Ok(())
}
