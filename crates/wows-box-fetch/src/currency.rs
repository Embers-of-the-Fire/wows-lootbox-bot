use serde::{Deserialize, Serialize};

use crate::item::ItemIcon;

const QUERY_CURRENCY: &str = r#"query Currencies($languageCode: String) {
    currencies(lang: $languageCode) {
        name
        title
        icons {
            default
            large
            small
        }
    }
}"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "snake_case")]
pub enum CurrencyType {
    /// 银币
    #[serde(rename = "credits")]
    Credits,
    /// 达布隆
    #[serde(rename = "gold")]
    Gold,
    /// 全局经验
    #[serde(rename = "free_xp")]
    FreeXp,
    /// 精英指挥官经验
    #[serde(rename = "elite_xp")]
    EliteXp,
    /// 研究点
    #[serde(rename = "paragon_xp")]
    ParagonXp,
    /// 钢铁
    #[serde(rename = "steel")]
    Steel,
    /// 煤炭
    #[serde(rename = "coal")]
    Coal,
    /// 军团代币
    #[serde(rename = "molybdenum")]
    Molybdenum,
    ///  
    #[serde(rename = "brass")]
    Brass,
    /// 网站筹码
    #[serde(rename = "saltpeter")]
    Saltpeter,
    /// 社区代币
    #[serde(rename = "recruitment_points")]
    RecruitmentPoints,
    /// 联邦代币
    #[serde(rename = "eventum_3")]
    Eventum3,
    /// 雄鹰代币
    #[serde(rename = "eventum_4")]
    Eventum4,
    /// 不列颠尼亚代币
    #[serde(rename = "eventum_5")]
    Eventum5,
    /// 缤纷代币
    #[serde(rename = "eventum_6")]
    Eventum6,
    /// 足球代币
    #[serde(rename = "eventum_7")]
    Eventum7,
    /// 夏季代币
    #[serde(rename = "eventum_8")]
    Eventum8,
    /// 航海家代币
    #[serde(rename = "eventum_9")]
    Eventum9,
    /// 新年证书
    #[serde(rename = "eventum_10")]
    Eventum10,
    /// 补给券
    #[serde(rename = "eventum_cn")]
    EventumCn,
    /// 志愿者代币
    #[serde(rename = "santium")]
    Santium,
    /// 造船阶段
    #[serde(rename = "dockyardum_1")]
    Dockyardum1,
    /// 造船阶段
    #[serde(rename = "dockyardum_2")]
    Dockyardum2,
    /// 蔚蓝档案代币
    #[serde(rename = "eventum_11")]
    Eventum11,
    /// 企业代币
    #[serde(rename = "eventum_12")]
    Eventum12,
    /// 节庆证书
    #[serde(rename = "eventum_13")]
    Eventum13,
    /// 圣诞代币
    #[serde(rename = "eventum_14")]
    Eventum14,
    /// 法国代币
    #[serde(rename = "eventum_1")]
    Eventum1,
    /// 金色代币
    #[serde(rename = "eventum_2")]
    Eventum2,
    /// 战斗代币
    #[serde(rename = "clientum_1")]
    Clientum1,
    ///  
    #[serde(rename = "clientum_2")]
    Clientum2,
    /// 石油
    #[serde(rename = "clan_resource")]
    ClanResource,

    // do not edit following
    /// 高级账号
    WowsPremium,

    /// 船坞
    Slots,

    /// Any other
    #[serde(other)]
    Verbatim,
}

impl CurrencyType {
    pub fn into_standard(self) -> Option<wows_box::currencies::CurrencyType> {
        use wows_box::currencies::CurrencyType::*;
        match self {
            Self::Credits => Some(Credits),
            Self::Gold => Some(Gold),
            Self::FreeXp => Some(FreeXp),
            Self::EliteXp => Some(EliteXp),
            Self::ParagonXp => Some(ParagonXp),
            Self::Steel => Some(Steel),
            Self::Coal => Some(Coal),
            Self::Molybdenum => Some(Molybdenum),
            Self::Brass => Some(Brass),
            Self::Saltpeter => Some(Saltpeter),
            Self::RecruitmentPoints => Some(RecruitmentPoints),
            Self::Eventum3 => Some(Eventum3),
            Self::Eventum4 => Some(Eventum4),
            Self::Eventum5 => Some(Eventum5),
            Self::Eventum6 => Some(Eventum6),
            Self::Eventum7 => Some(Eventum7),
            Self::Eventum8 => Some(Eventum8),
            Self::Eventum9 => Some(Eventum9),
            Self::Eventum10 => Some(Eventum10),
            Self::EventumCn => Some(EventumCn),
            Self::Santium => Some(Santium),
            Self::Dockyardum1 => Some(Dockyardum1),
            Self::Dockyardum2 => Some(Dockyardum2),
            Self::Eventum11 => Some(Eventum11),
            Self::Eventum12 => Some(Eventum12),
            Self::Eventum13 => Some(Eventum13),
            Self::Eventum14 => Some(Eventum14),
            Self::Eventum1 => Some(Eventum1),
            Self::Eventum2 => Some(Eventum2),
            Self::Clientum1 => Some(Clientum1),
            Self::Clientum2 => Some(Clientum2),
            Self::ClanResource => Some(ClanResource),

            // do not edit following
            Self::WowsPremium => Some(WowsPremium),
            Self::Slots => Some(Slots),
            Self::Verbatim => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrencyData {
    pub name: CurrencyType,
    pub title: String,
    pub icons: ItemIcon,
}

impl CurrencyData {
    pub fn into_standard(self, lang: &str) -> Option<wows_box::currencies::CurrencyData> {
        let icon = if let Some(icon) = self.icons.clone().into_large() {
            icon
        } else {
            self.icons.clone().into_default()
        };
        self.name
            .into_standard()
            .map(|name| wows_box::currencies::CurrencyData {
                r#type: name,
                name: name.as_name_string(lang).to_owned(),
                icon,
            })
    }
}

pub async fn fetch_currency_symbol(lang: &str) -> anyhow::Result<Vec<CurrencyData>> {
    let body = serde_json::json! {[{
        "query": QUERY_CURRENCY,
        "variables": {
            "languageCode": lang,
        }
    }]};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(non_camel_case_types)]
    struct CurrencyData__InternalBase {
        data: CurrencyData__InternalData,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(non_camel_case_types)]
    struct CurrencyData__InternalData {
        currencies: Vec<CurrencyData>,
    }

    let resp = reqwest::Client::new()
        .post("https://vortex.worldofwarships.asia/api/graphql/glossary/")
        .json(&body)
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to fetch currency data: {:?}", e))?;

    let content = resp
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to parse from json: {:?}", e))?;

    // dbg!(&content);

    let mut content: Vec<CurrencyData__InternalBase> = serde_json::from_value(content)?;

    let item = content
        .pop()
        .map(|t| t.data.currencies)
        .ok_or(anyhow::anyhow!("Failed to get a single result"))?;

    Ok(item.clone())
}

#[cfg(test)]
#[tokio::test]
async fn test_fetch_currency_symbol() -> anyhow::Result<()> {
    let res = fetch_currency_symbol("zh-sg").await?;
    println!("{:#?}", res);

    Ok(())
}

pub fn fetch_currency_image(
    currency: wows_box::currencies::CurrencyType,
    version_id: &str,
) -> anyhow::Result<String> {
    let curr_name = currency.as_icon_name();
    Ok(format!(
        "https://wows-web-static.wgcdn.co/wowsp/{}/browserAssets/img/{}.png",
        version_id, curr_name
    ))
}
