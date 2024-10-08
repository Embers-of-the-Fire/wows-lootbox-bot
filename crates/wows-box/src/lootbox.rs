use std::num::NonZeroU8;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LootBox {
    pub name: String,
    #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_default_from_null")]
    pub short_name: String,
    pub wows_name_id: String,
    pub id: u64,
    pub is_premium: bool,
    /// Only default icon
    pub icon: String,
    pub slots: Vec<LootBoxSlot>,
    pub filler: Option<LootBoxFiller>,
    /// guarantee
    pub save_point: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LootBoxFiller {
    pub filler: LootBoxRewardType,
    pub amount: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LootBoxSlot {
    pub common: Vec<LootBoxRewardList>,
    pub valuable: Vec<LootBoxRewardList>,
    #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_default_from_null")]
    pub name: String,
    pub continuous_rewards: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LootBoxRewardList {
    pub name: String,
    #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_default_from_null")]
    pub short_name: String,
    /// 0 ~ 1.0, 2 digits
    pub probability: f64,
    pub rewards: Vec<LootBoxReward>,
    pub has_unique_rewards: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LootBoxReward {
    pub probability: f64,
    pub amount: u32,
    pub reward: LootBoxRewardType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
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
    #[serde(rename = "camoboost")]
    CamoBoost { id: u64 },
    #[serde(rename = "collection_album")]
    CollectionAlbum { id: u64 },
    #[serde(rename = "signal")]
    Signal { id: u64, name: String },
    #[serde(rename_all = "camelCase")]
    #[serde(rename = "ship")]
    Ship {
        crew_level: Option<NonZeroU8>,
        ship_level: u8,
        id: u64,
        name: String,
        is_premium: bool,
        is_special: bool,
        icon: String,
    },
    #[serde(rename_all = "camelCase")]
    #[serde(rename = "skin")]
    Skin {
        id: u64,
        ship_id: u64,
        only_silver: bool,
        icon: String,
        name: String,
        ship: SkinShipBaseData,
    },
    #[serde(rename_all = "camelCase")]
    #[serde(rename = "camouflage")]
    Camouflage { id: u64, name: String, icon: String },
    #[serde(rename_all = "camelCase")]
    #[serde(rename = "permoflage")]
    Permoflage {
        id: u64,
        name: String,
        icon: String,
        is_native: bool,
        ship_id: u64,
        only_silver: bool,
        ship: SkinShipBaseData,
    },
    #[serde(rename_all = "camelCase")]
    #[serde(rename = "mskin")]
    Mskin {
        id: u64,
        name: String,
        icon: String,
        ship_id: u64,
        only_silver: bool,
        ship: SkinShipBaseData,
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
        name: String,
        icon: String,
    },
    #[serde(rename_all = "camelCase")]
    #[serde(rename = "multiboost")]
    Multiboost {
        id: u64,
        name: String,
        icon: String,
        restrictions: MultiboostRestriction,
    },
    #[serde(rename_all = "camelCase")]
    #[serde(rename = "ensign")]
    Ensign { id: u64, name: String, icon: String },
    #[serde(rename_all = "camelCase")]
    #[serde(rename = "lootbox")]
    Lootbox { id: u64, name: String, icon: String },
}

impl LootBoxRewardType {
    pub fn get_id(&self) -> Option<u64> {
        match self {
            Self::Credits
            | Self::Gold
            | Self::FreeXp
            | Self::EliteXp
            | Self::ParagonXp
            | Self::Steel
            | Self::Coal
            | Self::Molybdenum
            | Self::Brass
            | Self::Saltpeter
            | Self::RecruitmentPoints
            | Self::Eventum3
            | Self::Eventum4
            | Self::Eventum5
            | Self::Eventum6
            | Self::Eventum7
            | Self::Eventum8
            | Self::Eventum9
            | Self::Eventum10
            | Self::EventumCn
            | Self::Santium
            | Self::Dockyardum1
            | Self::Dockyardum2
            | Self::Eventum11
            | Self::Eventum12
            | Self::Eventum13
            | Self::Eventum14
            | Self::Eventum1
            | Self::Eventum2
            | Self::Clientum1
            | Self::Clientum2
            | Self::ClanResource
            // do not edit following
            | Self::Slots
            | Self::WowsPremium => None,
            Self::CamoBoost { id }
            | Self::CollectionAlbum { id }
            | Self::Signal { id, .. }
            | Self::Ship { id, .. }
            | Self::Skin { id, .. }
            | Self::Camouflage { id, .. }
            | Self::Permoflage { id, .. }
            | Self::Mskin { id, .. }
            | Self::Style { id, .. }
            | Self::Crew { id, .. }
            | Self::Multiboost { id, .. }
            | Self::Ensign { id, .. }
            | Self::Lootbox { id, .. } => Some(*id),
        }
    }

    pub fn as_precedence(&self) -> (u32, Option<u64>) {
        match self {
            Self::Lootbox { id, .. } => (0, Some(*id)),
            Self::Ship {
                crew_level,
                ship_level,
                is_premium,
                is_special,
                id,
                ..
            } => (
                1,
                Some(
                    (*is_special as u64).checked_shl(56).unwrap_or(0)
                        + (*is_premium as u64).checked_shl(48).unwrap_or(0)
                        + (*ship_level as u64).checked_shl(40).unwrap_or(0)
                        + (crew_level.map(|t| t.get()).unwrap_or_default() as u64)
                            .checked_shl(32)
                            .unwrap_or(0)
                        + (id.checked_shl(32).unwrap_or(0) >> 32),
                ),
            ),
            Self::Ensign { id, .. } => (2, Some(*id)),
            Self::Skin { id, ship, .. } => (3, Some(id_and_ship_seq(*id, ship))),
            Self::Permoflage { id, ship, .. } => (4, Some(id_and_ship_seq(*id, ship))),
            Self::Mskin { id, ship, .. } => (5, Some(id_and_ship_seq(*id, ship))),
            Self::Crew { id, crew_level, .. } => (
                6,
                Some(
                    (*crew_level as u64).checked_shl(56).unwrap_or(0)
                        + ((id.checked_shl(8).unwrap_or(0)) >> 8),
                ),
            ),
            Self::Multiboost {
                id, restrictions, ..
            } => (
                7,
                Some(
                    (*restrictions.levels.iter().max().unwrap_or(&0) as u64)
                        .checked_shl(56)
                        .unwrap_or(0)
                        + ((id.checked_shl(8).unwrap_or(0)) >> 8),
                ),
            ),
            Self::Steel => (8, None),
            Self::Gold => (9, None),
            Self::ParagonXp => (10, None),

            // other currencies, see `_ => ...`
            Self::Coal => (12, None),
            Self::FreeXp => (13, None),
            Self::EliteXp => (14, None),
            Self::RecruitmentPoints => (15, None),
            Self::WowsPremium => (16, None),
            Self::Slots => (17, None),
            Self::Credits => (18, None),
            Self::CamoBoost { id } => (19, Some(*id)),
            Self::CollectionAlbum { id } => (20, Some(*id)),
            Self::Signal { id, .. } => (21, Some(*id)),
            Self::Camouflage { id, .. } => (22, Some(*id)),
            Self::Style { id, .. } => (23, Some(*id)),

            _ => (11, None),
        }
    }
}

fn id_and_ship_seq(id: u64, ship: &SkinShipBaseData) -> u64 {
    ((ship.is_special as u64).checked_shl(56).unwrap_or(0)
        + (ship.is_premium as u64).checked_shl(48).unwrap_or(0)
        + (ship.ship_level as u64).checked_shl(40).unwrap_or(0)
        + (id.checked_shl(24).unwrap_or(0)))
        >> 24
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "camelCase")]
pub struct SkinShipBaseData {
    pub id: u64,
    pub name: String,
    pub ship_level: u8,
    pub is_premium: bool,
    pub is_special: bool,
    pub icon: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "camelCase")]
pub struct MultiboostRestriction {
    pub levels: Vec<u8>,
}
