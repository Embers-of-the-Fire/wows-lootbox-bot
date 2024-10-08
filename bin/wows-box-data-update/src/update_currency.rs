use std::{collections::HashMap, env, time::Instant};

use bson::doc;
use log::{debug, info};
use mongodb::{Client, Collection};

use wows_box::currencies::{CurrencyData, CurrencyType};
use wows_box_fetch::currency::{fetch_currency_image, fetch_currency_symbol};

pub async fn update_currency(lang: &str, client: &Client) -> anyhow::Result<()> {
    info!("Started updating wows currencies data [lang {}]...", lang);
    let time_c = Instant::now();

    let box_db = client.database(&format!("wowslootbox-{lang}"));
    let currency_collection: Collection<CurrencyData> = box_db.collection("currencies");

    let curr_wows_web_version = env::var("WOWS_WEB_VERSION")?;

    let data = fetch_currency_symbol(lang)
        .await?
        .into_iter()
        .filter_map(|t| {
            t.name
                .into_standard()
                .and_then(|n| t.into_standard(lang).map(|s| (n, s)))
        })
        .collect::<HashMap<_, _>>();
    debug!("Fetched wows currencies data: {:?}", data);
    for item in CurrencyType::ALL_QUERY_CURRENCIES {
        debug!("Started processing currency {:?}", item);
        // let data = CurrencyData {
        //     r#type: item,
        //     name: item.as_name_string().to_owned(),
        //     icon: fetch_currency_image(item, curr_wows_web_version)?,
        // };
        let res = currency_collection
            .find_one(doc! { "type": item.as_icon_name() })
            .await?;
        if res.is_some() {
            debug!("Duplicate item: {:?}", item);
            continue;
        }
        let d = data.get(&item).unwrap();
        currency_collection.insert_one(d).await?;
    }
    for item in CurrencyType::ALL_NON_QUERY_CURRENCIES {
        debug!("Started processing currency {:?}", item);
        let res = currency_collection
            .find_one(doc! { "type": item.as_icon_name() })
            .await?;
        if res.is_some() {
            debug!("Duplicate item: {:?}", item);
            continue;
        }
        let d = CurrencyData {
            r#type: item,
            name: item.as_name_string(lang).to_owned(),
            icon: fetch_currency_image(item, &curr_wows_web_version)?,
        };
        currency_collection.insert_one(d).await?;
    }

    info!(
        "Updated wows currencies data in {:.2}s",
        time_c.elapsed().as_secs_f64()
    );

    Ok(())
}
