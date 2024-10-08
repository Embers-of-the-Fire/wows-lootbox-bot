use std::{collections::HashMap, num::NonZeroU8};

use serde::{Deserialize, Serialize};

use crate::item::ItemIcon;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LootBox {
    pub title: String,
    #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_default_from_null")]
    pub short_title: String,
    pub id: u64,
    pub name: String,
    pub is_premium: bool,
    pub icons: ItemIcon,
    pub slots: Vec<LootBoxSlot>,
    pub filler: Option<LootBoxFiller>,
}

impl LootBox {
    pub fn into_standrad(self) -> wows_box::lootbox::LootBox {
        let filler = if let Some(filler) = self.filler.clone() {
            Some(filler.into_standard(&self))
        } else {
            None
        };
        let save_point = self
            .slots
            .iter()
            .flat_map(|t| t.common_rewards.values().chain(t.valuable_rewards.values()))
            .filter_map(|t| t.save_point)
            .max();
        wows_box::lootbox::LootBox {
            name: self.title,
            short_name: self.short_title,
            wows_name_id: self.name,
            id: self.id,
            is_premium: self.is_premium,
            icon: self.icons.into_default(),
            slots: self
                .slots
                .into_iter()
                .map(|slot| slot.into_standard())
                .collect(),
            filler,
            save_point,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LootBoxFiller {
    #[serde(flatten)]
    pub filler: LootBoxFillerType,
    pub amount: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum LootBoxFillerType {
    #[serde(rename = "credits")]
    Credits,
    #[serde(rename = "gold")]
    Gold,
    #[serde(rename = "wows_premium")]
    WowsPremium,
    #[serde(rename = "steel")]
    Steel,
    #[serde(rename = "coal")]
    Coal,
    #[serde(rename = "paragon_xp")]
    ParagonXp,
    #[serde(rename = "free_xp")]
    FreeXp,
    #[serde(rename = "elite_xp")]
    EliteXp,
    #[serde(rename = "slots")]
    Slots,
    #[serde(rename = "recruitment_points")]
    RecruitmentPoints,
    #[serde(rename = "camoboost")]
    CamoBoost { id: u64 },
    #[serde(rename = "collection_album")]
    CollectionAlbum { id: u64 },
    #[serde(rename = "signal")]
    Signal { id: u64 },
    #[serde(rename_all = "camelCase")]
    #[serde(rename = "camouflage")]
    Camouflage { id: u64 },
    #[serde(rename_all = "camelCase")]
    #[serde(rename = "style")]
    Style { id: u64 },
}

impl LootBoxFiller {
    pub fn into_standard(self, glob: &LootBox) -> wows_box::lootbox::LootBoxFiller {
        use wows_box::lootbox::LootBoxRewardType::*;
        wows_box::lootbox::LootBoxFiller {
            filler: match self.filler {
                LootBoxFillerType::Credits => Credits,
                LootBoxFillerType::Gold => Gold,
                LootBoxFillerType::WowsPremium => WowsPremium,
                LootBoxFillerType::Steel => Steel,
                LootBoxFillerType::Coal => Coal,
                LootBoxFillerType::ParagonXp => ParagonXp,
                LootBoxFillerType::FreeXp => FreeXp,
                LootBoxFillerType::EliteXp => EliteXp,
                LootBoxFillerType::Slots => Slots,
                LootBoxFillerType::RecruitmentPoints => RecruitmentPoints,
                LootBoxFillerType::CamoBoost { id } => CamoBoost { id },
                LootBoxFillerType::CollectionAlbum { id } => CollectionAlbum { id },
                LootBoxFillerType::Signal { id } | LootBoxFillerType::Camouflage { id } => glob
                    .slots
                    .iter()
                    .flat_map(|t| t.common_rewards.values().chain(t.valuable_rewards.values()))
                    .flat_map(|t| &t.rewards)
                    .find(|t| {
                        (*t).to_owned()
                            .into_standard(0.0)
                            .reward
                            .get_id()
                            .is_some_and(|i| i == id)
                    })
                    .unwrap()
                    .reward
                    .clone()
                    .into_standard(),
                LootBoxFillerType::Style { id } => Style { id },
            },
            amount: self.amount,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LootBoxSlot {
    pub common_rewards: HashMap<String, LootBoxRewardList>,
    pub continuous_rewards: bool,
    pub valuable_rewards: HashMap<String, LootBoxRewardList>,
    pub title: String,
}

impl LootBoxSlot {
    pub fn into_standard(self) -> wows_box::lootbox::LootBoxSlot {
        wows_box::lootbox::LootBoxSlot {
            name: self.title,
            continuous_rewards: self.continuous_rewards,
            common: self
                .common_rewards
                .into_values()
                .map(|list| list.into_standard())
                .collect(),
            valuable: self
                .valuable_rewards
                .into_values()
                .map(|list| list.into_standard())
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LootBoxRewardList {
    pub title: String,
    #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_default_from_null")]
    pub short_title: String,
    #[serde(flatten, default)]
    pub probability: Option<RewardProbability>,
    pub save_point: Option<u32>,
    #[serde(default = "bool_false")]
    pub has_unique_rewards: bool,
    #[serde(default)]
    pub rewards: Vec<LootBoxReward>,
}

impl LootBoxRewardList {
    pub fn into_standard(self) -> wows_box::lootbox::LootBoxRewardList {
        let rate = if let Some(p) = self.probability {
            p.probability_displayed / 100.0
        } else {
            self.rewards
                .iter()
                .filter_map(|t| t.probability)
                .map(|t| t.probability_displayed / 100.0)
                .sum()
        };
        let no_rate_num = self
            .rewards
            .iter()
            .filter(|t| t.probability.is_none())
            .count();
        wows_box::lootbox::LootBoxRewardList {
            name: self.title,
            short_name: self.short_title,
            probability: rate,
            has_unique_rewards: self.has_unique_rewards,
            rewards: self
                .rewards
                .into_iter()
                .map(|r| r.into_standard(rate / (no_rate_num as f64)))
                .collect(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RewardProbability {
    #[serde(
        deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string",
        default
    )]
    pub probability: f64,
    pub weight: u32,
    pub probability_displayed: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LootBoxReward {
    #[serde(default, flatten)]
    pub probability: Option<RewardProbability>,
    pub amount: u32,
    #[serde(flatten)]
    pub reward: LootBoxRewardType,
}

impl LootBoxReward {
    pub fn into_standard(self, rate: f64) -> wows_box::lootbox::LootBoxReward {
        wows_box::lootbox::LootBoxReward {
            probability: if let Some(p) = self.probability {
                p.probability_displayed / 100.0
            } else {
                rate
            },
            amount: self.amount,
            reward: self.reward.into_standard(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum LootBoxRewardType {
    #[serde(rename = "credits")]
    Credits,
    #[serde(rename = "gold")]
    Gold,
    #[serde(rename = "free_xp")]
    FreeXp,
    #[serde(rename = "elite_xp")]
    EliteXp,
    #[serde(rename = "paragon_xp")]
    ParagonXp,
    #[serde(rename = "steel")]
    Steel,
    #[serde(rename = "coal")]
    Coal,
    #[serde(rename = "molybdenum")]
    Molybdenum,
    #[serde(rename = "brass")]
    Brass,
    #[serde(rename = "saltpeter")]
    Saltpeter,
    #[serde(rename = "recruitment_points")]
    RecruitmentPoints,
    #[serde(rename = "eventum_3")]
    Eventum3,
    #[serde(rename = "eventum_4")]
    Eventum4,
    #[serde(rename = "eventum_5")]
    Eventum5,
    #[serde(rename = "eventum_6")]
    Eventum6,
    #[serde(rename = "eventum_7")]
    Eventum7,
    #[serde(rename = "eventum_8")]
    Eventum8,
    #[serde(rename = "eventum_9")]
    Eventum9,
    #[serde(rename = "eventum_10")]
    Eventum10,
    #[serde(rename = "eventum_cn")]
    EventumCn,
    #[serde(rename = "santium")]
    Santium,
    #[serde(rename = "dockyardum_1")]
    Dockyardum1,
    #[serde(rename = "dockyardum_2")]
    Dockyardum2,
    #[serde(rename = "eventum_11")]
    Eventum11,
    #[serde(rename = "eventum_12")]
    Eventum12,
    #[serde(rename = "eventum_13")]
    Eventum13,
    #[serde(rename = "eventum_14")]
    Eventum14,
    #[serde(rename = "eventum_1")]
    Eventum1,
    #[serde(rename = "eventum_2")]
    Eventum2,
    #[serde(rename = "clientum_1")]
    Clientum1,
    #[serde(rename = "clientum_2")]
    Clientum2,
    #[serde(rename = "clan_resource")]
    ClanResource,

    // do not edit following
    #[serde(rename = "slots")]
    Slots,
    #[serde(rename = "wows_premium")]
    WowsPremium,
    // do not edit above
    #[serde(rename_all = "camelCase")]
    #[serde(rename = "camoboost")]
    CamoBoost { id: u64 },
    #[serde(rename = "collection_album")]
    CollectionAlbum { id: u64 },
    #[serde(rename_all = "camelCase")]
    #[serde(rename = "signal")]
    Signal {
        id: u64,
        additional_data: SignalData,
    },
    #[serde(rename_all = "camelCase")]
    #[serde(rename = "ship")]
    Ship {
        crew_level: Option<NonZeroU8>,
        id: u64,
        additional_data: ShipData,
    },
    #[serde(rename_all = "camelCase")]
    #[serde(rename = "skin")]
    Skin {
        id: u64,
        ship_id: u64,
        only_silver: bool,
        additional_data: SkinData,
    },
    #[serde(rename_all = "camelCase")]
    #[serde(rename = "camouflage")]
    Camouflage {
        id: u64,
        additional_data: CamouflageData,
    },
    #[serde(rename_all = "camelCase")]
    #[serde(rename = "permoflage")]
    Permoflage {
        id: u64,
        ship_id: u64,
        only_silver: bool,
        additional_data: PermoflageData,
    },
    #[serde(rename_all = "camelCase")]
    #[serde(rename = "mskin")]
    Mskin {
        id: u64,
        ship_id: u64,
        only_silver: bool,
        additional_data: MskinData,
    },
    #[serde(rename_all = "camelCase")]
    #[serde(rename = "style")]
    Style { id: u64 },
    #[serde(rename_all = "camelCase")]
    #[serde(rename = "crew")]
    Crew {
        id: u64,
        ship_id: u64,
        crew_level: u8,
        additional_data: CrewData,
    },
    #[serde(rename_all = "camelCase")]
    #[serde(rename = "multiboost")]
    Mulitboost {
        id: u64,
        additional_data: MulitboostData,
    },
    #[serde(rename_all = "camelCase")]
    #[serde(rename = "ensign")]
    Ensign {
        id: u64,
        additional_data: EnsignData,
    },
    #[serde(rename_all = "camelCase")]
    #[serde(rename = "lootbox")]
    Lootbox {
        id: u64,
        additional_data: LootboxData,
    },
}

impl LootBoxRewardType {
    pub fn into_standard(self) -> wows_box::lootbox::LootBoxRewardType {
        use wows_box::lootbox::LootBoxRewardType::*;

        match self {
            Self::Credits => Credits,
            Self::Gold => Gold,
            Self::FreeXp => FreeXp,
            Self::EliteXp => EliteXp,
            Self::ParagonXp => ParagonXp,
            Self::Steel => Steel,
            Self::Coal => Coal,
            Self::Molybdenum => Molybdenum,
            Self::Brass => Brass,
            Self::Saltpeter => Saltpeter,
            Self::RecruitmentPoints => RecruitmentPoints,
            Self::Eventum3 => Eventum3,
            Self::Eventum4 => Eventum4,
            Self::Eventum5 => Eventum5,
            Self::Eventum6 => Eventum6,
            Self::Eventum7 => Eventum7,
            Self::Eventum8 => Eventum8,
            Self::Eventum9 => Eventum9,
            Self::Eventum10 => Eventum10,
            Self::EventumCn => EventumCn,
            Self::Santium => Santium,
            Self::Dockyardum1 => Dockyardum1,
            Self::Dockyardum2 => Dockyardum2,
            Self::Eventum11 => Eventum11,
            Self::Eventum12 => Eventum12,
            Self::Eventum13 => Eventum13,
            Self::Eventum14 => Eventum14,
            Self::Eventum1 => Eventum1,
            Self::Eventum2 => Eventum2,
            Self::Clientum1 => Clientum1,
            Self::Clientum2 => Clientum2,
            Self::ClanResource => ClanResource,

            // do not edit following
            Self::WowsPremium => WowsPremium,
            Self::Slots => Slots,

            Self::CamoBoost { id } => CamoBoost { id },
            Self::CollectionAlbum { id } => CollectionAlbum { id },
            Self::Signal {
                id,
                additional_data,
            } => Signal {
                id,
                name: additional_data.title,
            },
            Self::Ship {
                id,
                crew_level,
                additional_data,
            } => Ship {
                id,
                crew_level,
                ship_level: additional_data.level,
                name: additional_data.title,
                is_premium: additional_data.is_premium,
                is_special: additional_data.is_special,
                icon: additional_data.icons.into_default(),
            },
            Self::Skin {
                id,
                ship_id,
                only_silver,
                additional_data,
            } => Skin {
                id,
                ship_id,
                only_silver,
                name: additional_data.title,
                ship: additional_data.ship.into_standard(),
                icon: additional_data.icons.into_default(),
            },
            Self::Camouflage {
                id,
                additional_data,
            } => Camouflage {
                id,
                name: additional_data.title,
                icon: additional_data.icons.into_default(),
            },
            Self::Permoflage {
                id,
                ship_id,
                only_silver,
                additional_data,
            } => Permoflage {
                id,
                name: additional_data.title,
                icon: additional_data.icons.into_default(),
                is_native: additional_data.is_native,
                ship_id,
                only_silver,
                ship: additional_data.ship.into_standard(),
            },
            Self::Mskin {
                id,
                ship_id,
                only_silver,
                additional_data,
            } => Mskin {
                id,
                name: additional_data.title,
                icon: additional_data.icons.into_default(),
                ship_id,
                only_silver,
                ship: additional_data.ship.into_standard(),
            },
            Self::Style {
                id,
            } => Style {
                id,
            },
            Self::Crew {
                id,
                ship_id,
                crew_level,
                additional_data,
            } => Crew {
                id,
                ship_id,
                crew_level,
                name: additional_data.title,
                icon: additional_data.icons.into_default(),
            },
            Self::Mulitboost {
                id,
                additional_data,
            } => Multiboost {
                id,
                name: additional_data.title,
                icon: additional_data.icons.into_default(),
                restrictions: wows_box::lootbox::MultiboostRestriction {
                    levels: additional_data.restrictions.levels,
                },
            },
            Self::Ensign {
                id,
                additional_data,
            } => Ensign {
                id,
                name: additional_data.title,
                icon: additional_data.icons.into_default(),
            },
            Self::Lootbox {
                id,
                additional_data,
            } => Lootbox {
                id,
                name: additional_data.title,
                icon: additional_data.icons.into_default(),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalData {
    pub title: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShipData {
    pub title: String,
    pub level: u8,
    pub is_premium: bool,
    pub is_special: bool,
    pub icons: ItemIcon,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkinData {
    pub title: String,
    pub icons: ItemIcon,
    pub ship: SkinShipBaseData,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkinShipBaseData {
    pub id: u64,
    pub title: String,
    pub level: u8,
    pub is_premium: bool,
    pub is_special: bool,
    pub icons: ItemIcon,
}

impl SkinShipBaseData {
    pub fn into_standard(self) -> wows_box::lootbox::SkinShipBaseData {
        wows_box::lootbox::SkinShipBaseData {
            id: self.id,
            name: self.title,
            ship_level: self.level,
            is_premium: self.is_premium,
            is_special: self.is_special,
            icon: self.icons.into_default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CamouflageData {
    pub title: String,
    pub icons: ItemIcon,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PermoflageData {
    pub title: String,
    pub icons: ItemIcon,
    pub is_native: bool,
    pub ship: SkinShipBaseData,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MskinData {
    pub title: String,
    pub icons: ItemIcon,
    pub ship: SkinShipBaseData,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrewData {
    pub title: String,
    pub icons: ItemIcon,
    pub is_unique: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MulitboostData {
    pub title: String,
    pub icons: ItemIcon,
    pub restrictions: MultiboostRestriction,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiboostRestriction {
    pub specific_ships: Vec<u64>,
    pub forbidden_ships: Vec<u64>,
    pub levels: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnsignData {
    pub title: String,
    pub icons: ItemIcon,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LootboxData {
    pub title: String,
    pub icons: ItemIcon,
}

const fn bool_false() -> bool {
    false
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "camelCase")]
pub enum LootBoxFetchResponse {
    #[serde(rename = "ok")]
    Ok { data: LootBox },
    #[serde(rename = "error", other)]
    Error,
}

impl LootBoxFetchResponse {
    pub fn is_ok(&self) -> bool {
        match self {
            LootBoxFetchResponse::Ok { .. } => true,
            _ => false,
        }
    }

    pub fn is_err(&self) -> bool {
        !self.is_ok()
    }

    pub fn ok(self) -> Option<LootBox> {
        match self {
            LootBoxFetchResponse::Ok { data } => Some(data),
            _ => None,
        }
    }
}

pub async fn fetch_lootbox(lang: &str, id: u64) -> anyhow::Result<LootBoxFetchResponse> {
    let resp = reqwest::get(format!(
        "https://vortex.worldofwarships.asia/api/get_lootbox/{lang}/{id}/"
    ))
    .await
    .map_err(|e| anyhow::anyhow!("Failed to fetch lootbox detail: {:?}", e))?;

    let lootbox = resp
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to parse json: {:?}", e))?;

    // dbg!(&lootbox);
    // let lootbox = serde_json::from_value(lootbox)
    //     .map_err(|e| anyhow::anyhow!("Failed to parse json: {:?}", e))?;

    Ok(lootbox)
}

#[cfg(test)]
#[tokio::test]
async fn test_fetch_box() {
    let lootbox = fetch_lootbox("zh-sg", 4184003504).await.unwrap();
    println!("{:#?}", lootbox);
}
