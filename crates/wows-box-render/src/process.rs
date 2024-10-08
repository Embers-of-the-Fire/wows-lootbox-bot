use std::{collections::HashMap, env, fs};

use anyhow::anyhow;
use bson::doc;
use itertools::Itertools;
use lazy_static::lazy_static;
use log::debug;
use minijinja::{Environment, Template, Value};
use mongodb::{Client, Collection};
use rand::{rngs::SmallRng, SeedableRng};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use wows_box::{
    currencies::{CurrencyData, CurrencyType},
    item::ItemData,
    lootbox::{LootBox, LootBoxRewardType},
};
use wows_box_rand::rand::rand_multi;

use crate::html::render_html;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LootBoxListProp {
    pub box_icon: String,
    pub box_title: String,
    pub rewards: Vec<LootBoxListRewardProp>,
    pub times: u32,
    pub guarantee_text: &'static str,
}

pub fn guarantee_text(lang: &str) -> &'static str {
    match lang {
        "zh-sg" => "（保底）",
        _ => "(Guaranteed)",
    }
}

impl LootBoxListProp {
    pub async fn from_result(
        lang: &str,
        db: &Client,
        box_id: u64,
        result: HashMap<(LootBoxRewardType, bool), u32>,
        times: u32,
    ) -> anyhow::Result<Self> {
        let box_db = db.database(&format!("wowslootbox-{lang}"));
        let currency_col: Collection<CurrencyData> = box_db.collection("currencies");
        let item_col: Collection<ItemData> = box_db.collection("items");
        let list_col: Collection<LootBox> = box_db.collection("list");

        debug!("Fetching lootbox data...");
        let box_data = list_col
            .find_one(doc! { "id": box_id as u32 })
            .await?
            .ok_or(anyhow!("Cannot find lootbox {}", box_id))?;
        let box_icon = box_data.icon;
        let box_title = box_data.name;

        debug!("Fetching reward data...");
        let mut vec = vec![];
        for ((reward, guarantee), amount) in result.into_iter() {
            let p = reward.as_precedence();
            let (name, img) = reward_to_imgs(lang, &currency_col, &item_col, reward).await?;
            vec.push(LootBoxListRewardProp {
                icons: img,
                text: name,
                amount,
                precedence: p,
                is_guaranteed: guarantee,
            });
        }

        vec.sort_by_key(|t| {
            0u128
                + (t.precedence.0 as u128).checked_shl(64).unwrap_or(0)
                + t.precedence.1.unwrap_or(0) as u128
        });

        Ok(LootBoxListProp {
            box_icon,
            box_title,
            rewards: vec,
            times,
            guarantee_text: guarantee_text(lang),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LootBoxListRewardProp {
    pub icons: Vec<String>,
    pub text: String,
    pub amount: u32,
    pub precedence: (u32, Option<u64>),
    pub is_guaranteed: bool,
}

async fn reward_to_imgs(
    lang: &str,
    curr_col: &Collection<CurrencyData>,
    item_col: &Collection<ItemData>,
    reward: LootBoxRewardType,
) -> anyhow::Result<(String, Vec<String>)> {
    use LootBoxRewardType::*;
    match reward {
        Credits => currency_to_img(curr_col, CurrencyType::Credits.as_icon_name()).await,
        Gold => currency_to_img(curr_col, CurrencyType::Gold.as_icon_name()).await,
        FreeXp => currency_to_img(curr_col, CurrencyType::FreeXp.as_icon_name()).await,
        EliteXp => currency_to_img(curr_col, CurrencyType::EliteXp.as_icon_name()).await,
        ParagonXp => currency_to_img(curr_col, CurrencyType::ParagonXp.as_icon_name()).await,
        Steel => currency_to_img(curr_col, CurrencyType::Steel.as_icon_name()).await,
        Coal => currency_to_img(curr_col, CurrencyType::Coal.as_icon_name()).await,
        Molybdenum => currency_to_img(curr_col, CurrencyType::Molybdenum.as_icon_name()).await,
        Brass => currency_to_img(curr_col, CurrencyType::Brass.as_icon_name()).await,
        Saltpeter => currency_to_img(curr_col, CurrencyType::Saltpeter.as_icon_name()).await,
        RecruitmentPoints => {
            currency_to_img(curr_col, CurrencyType::RecruitmentPoints.as_icon_name()).await
        }
        Eventum3 => currency_to_img(curr_col, CurrencyType::Eventum3.as_icon_name()).await,
        Eventum4 => currency_to_img(curr_col, CurrencyType::Eventum4.as_icon_name()).await,
        Eventum5 => currency_to_img(curr_col, CurrencyType::Eventum5.as_icon_name()).await,
        Eventum6 => currency_to_img(curr_col, CurrencyType::Eventum6.as_icon_name()).await,
        Eventum7 => currency_to_img(curr_col, CurrencyType::Eventum7.as_icon_name()).await,
        Eventum8 => currency_to_img(curr_col, CurrencyType::Eventum8.as_icon_name()).await,
        Eventum9 => currency_to_img(curr_col, CurrencyType::Eventum9.as_icon_name()).await,
        Eventum10 => currency_to_img(curr_col, CurrencyType::Eventum10.as_icon_name()).await,
        EventumCn => currency_to_img(curr_col, CurrencyType::EventumCn.as_icon_name()).await,
        Santium => currency_to_img(curr_col, CurrencyType::Santium.as_icon_name()).await,
        Dockyardum1 => currency_to_img(curr_col, CurrencyType::Dockyardum1.as_icon_name()).await,
        Dockyardum2 => currency_to_img(curr_col, CurrencyType::Dockyardum2.as_icon_name()).await,
        Eventum11 => currency_to_img(curr_col, CurrencyType::Eventum11.as_icon_name()).await,
        Eventum12 => currency_to_img(curr_col, CurrencyType::Eventum12.as_icon_name()).await,
        Eventum13 => currency_to_img(curr_col, CurrencyType::Eventum13.as_icon_name()).await,
        Eventum14 => currency_to_img(curr_col, CurrencyType::Eventum14.as_icon_name()).await,
        Eventum1 => currency_to_img(curr_col, CurrencyType::Eventum1.as_icon_name()).await,
        Eventum2 => currency_to_img(curr_col, CurrencyType::Eventum2.as_icon_name()).await,
        Clientum1 => currency_to_img(curr_col, CurrencyType::Clientum1.as_icon_name()).await,
        Clientum2 => currency_to_img(curr_col, CurrencyType::Clientum2.as_icon_name()).await,
        ClanResource => currency_to_img(curr_col, CurrencyType::ClanResource.as_icon_name()).await,

        // do not edit following
        WowsPremium => currency_to_img(curr_col, "wows-premium").await,
        Slots => currency_to_img(curr_col, "slots").await,

        CamoBoost { id } | CollectionAlbum { id } | Signal { id, .. } | Style { id } => {
            item_to_img(item_col, id).await
        }
        Ship {
            crew_level,
            ship_level,
            name,
            icon,
            ..
        } => Ok((
            format!(
                "{} {name}{}",
                level_to_str(ship_level),
                if let Some(level) = crew_level {
                    format!("（{level} {}）", 
                    match lang {
                        "zh-sg" => "级舰长",
                        _ => "Lv. Crew",
                    })
                } else {
                    "".to_owned()
                }
            ),
            vec![icon],
        )),
        Skin {
            name, icon, ship, ..
        } => Ok((
            format!("{} - {} {}", name, level_to_str(ship.ship_level), ship.name),
            vec![icon, ship.icon],
        )),
        Camouflage { name, icon, .. } => Ok((name, vec![icon])),
        Permoflage {
            name, icon, ship, ..
        } => Ok((
            format!("{} - {} {}", name, level_to_str(ship.ship_level), ship.name),
            vec![icon, ship.icon],
        )),
        Mskin {
            name, icon, ship, ..
        } => Ok((
            format!("{} - {} {}", name, level_to_str(ship.ship_level), ship.name),
            vec![icon, ship.icon],
        )),
        Crew {
            crew_level,
            name,
            icon,
            ..
        } => Ok((
            format!(
                "{crew_level}{} {name}",
                match lang {
                    "zh-sg" => "级舰长",
                    _ => " Lv.",
                }
            ),
            vec![icon],
        )),
        Multiboost {
            icon, restrictions, ..
        } => Ok((
            format!(
                "{}级加成包",
                restrictions
                    .levels
                    .iter()
                    .sorted()
                    .map(|t| level_to_str(*t))
                    .join("、")
            ),
            vec![icon],
        )),
        Ensign { name, icon, .. } | Lootbox { name, icon, .. } => Ok((name, vec![icon])),
    }
}

/// Return type: Name, Icon URL
async fn currency_to_img(
    col: &Collection<CurrencyData>,
    curr: &str,
) -> anyhow::Result<(String, Vec<String>)> {
    let data = col
        .find_one(doc! { "type": curr })
        .await?
        .ok_or(anyhow!("Cannot find currency {}", curr))?;

    Ok((data.name, vec![data.icon]))
}

/// Return type: Name, Icon URL
async fn item_to_img(
    col: &Collection<ItemData>,
    item_id: u64,
) -> anyhow::Result<(String, Vec<String>)> {
    let data = col
        .find_one(doc! { "id": item_id as u32 })
        .await?
        .ok_or(anyhow!("Cannot find item {}", item_id))?;

    Ok((data.name, vec![data.icon]))
}

const fn level_to_str(level: u8) -> &'static str {
    match level {
        1 => "I",
        2 => "II",
        3 => "III",
        4 => "IV",
        5 => "V",
        6 => "VI",
        7 => "VII",
        8 => "VIII",
        9 => "IX",
        10 => "X",
        11 => "⭐",
        _ => "",
    }
}

fn chunks(value: Vec<Value>, chunk_size: usize) -> Vec<Vec<Value>> {
    value
        .into_iter()
        .chunks(chunk_size)
        .into_iter()
        .map(|t| t.collect_vec())
        .collect_vec()
}

fn env(value: &str) -> String {
    env::var(value).unwrap_or_default()
}

lazy_static! {
    pub static ref JINJA_ENVIRONMENT: Environment<'static> = {
        let mut e = Environment::new();

        e.add_filter("chunks", chunks);

        e.add_function("env", env);

        e.add_template(
            "lootbox",
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/templates/lootbox.jinja"
            )),
        )
        .unwrap();

        e
    };
    pub static ref LOOTBOX_TEMPLATE: Template<'static, 'static> =
        JINJA_ENVIRONMENT.get_template("lootbox").unwrap();
}

pub async fn render_to_file(
    lang: &str,
    client: &Client,
    key: u64,
    times: u32,
) -> anyhow::Result<String> {
    let col: Collection<LootBox> = client
        .database(&format!("wowslootbox-{lang}"))
        .collection("list");
    let lootbox = col.find_one(doc! { "id": key as u32 }).await?.unwrap();

    let found = vec![];

    let mut rng = SmallRng::from_entropy();

    let resp = rand_multi(&mut rng, &lootbox, times, &found, 0);

    let list_prop = LootBoxListProp::from_result(lang, client, key, resp, times).await?;

    let uuid = Uuid::new_v4();
    let cache_html_file_path = format!("{}/{}.html", env::var("CACHE_DIR")?, uuid);
    let file_path = format!("{}/{}.png", env::var("CACHE_DIR")?, uuid);

    LOOTBOX_TEMPLATE.render_to_write(list_prop, fs::File::create(&cache_html_file_path)?)?;

    render_html(cache_html_file_path, &file_path, "table#list", "div#loaded")?;

    Ok(file_path)
}
