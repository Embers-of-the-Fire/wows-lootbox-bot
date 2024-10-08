//! Fetch wows lootbox list.

use serde::{Serialize, Deserialize};

const QUERY_LOOTBOX_LIST: &str = r#"query Lootbox($languageCode: String!) {
    lootbox(lang: $languageCode) {
        id
        isPremium
        name
        title
        shortTitle
    }
}"#;

/// Lootbox list item.
///
/// Note that the deserialization of this item is different from
/// the serialization of it.
///
/// Given that we are restricted to `String`s when parsing html contents,
/// we have to convert the text to the value we want.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LootboxListItem {
    #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
    pub id: u64,
    pub is_premium: bool,
    pub name: String,
    pub title: String,
    pub short_title: String,
}

pub async fn fetch_list(lang: &str) -> anyhow::Result<Vec<LootboxListItem>> {
    let body = serde_json::json! {[{
        "query": QUERY_LOOTBOX_LIST,
        "variables": {
            "languageCode": lang,
        }
    }]};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(non_camel_case_types)]
    struct LootboxList__InternalBase {
        data: LootboxList__InternalBoxList,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(non_camel_case_types)]
    struct LootboxList__InternalBoxList {
        lootbox: Vec<LootboxListItem>,
    }

    
    let resp = reqwest::Client::new()
        .post("https://vortex.worldofwarships.asia/api/graphql/glossary/")
        .json(&body)
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to fetch lootbox list data: {:?}", e))?;

    let content = resp.json().await.map_err(|e| anyhow::anyhow!("Failed to parse from json: {:?}", e))?;

    let mut content: Vec<LootboxList__InternalBase> = serde_json::from_value(content)?;

    let item = content
        .pop()
        .map(|t| t.data.lootbox)
        .ok_or(anyhow::anyhow!("Failed to get a single result"))?;

    Ok(item.clone())
}

#[cfg(test)]
#[tokio::test]
async fn test_fetch_list() {
    let list = fetch_list("zh-sg").await.unwrap();
    println!("{:#?}", list);
}
