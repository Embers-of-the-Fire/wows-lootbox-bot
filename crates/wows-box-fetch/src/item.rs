use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemIcon {
    pub small: Option<String>,
    pub large: Option<String>,
    pub default: String,
}

impl ItemIcon {
    pub fn into_default(self) -> String {
        Self::process(self.default)
    }
    pub fn into_small(self) -> Option<String> {
        self.small.map(Self::process)
    }
    pub fn into_large(self) -> Option<String> {
        self.large.map(Self::process)
    }

    fn process(item: String) -> String {
        if item.starts_with("https") {
            item
        } else if item.starts_with("//") {
            format!("https:{}", item)
        } else {
            format!("https://wows-gloss-icons.wgcdn.co/icons/{}", item)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemData {
    pub title: String,
    #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_default_from_null")]
    pub description: String,
    #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
    pub id: u64,
    #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_default_from_null")]
    pub title_short: String,
    pub type_name: String,
    pub icons: ItemIcon,
    pub r#type: ItemType,
}

impl ItemData {
    pub fn into_standard(self) -> wows_box::item::ItemData {
        wows_box::item::ItemData {
            name: self.title,
            short_name: self.title_short,
            id: self.id,
            icon: self.icons.into_default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemType {
    pub name: String,
    #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_default_from_null")]
    pub title: String,
}

const QUERY_ITEMS: &str = r#"query Items($languageCode: String, $id: String) {
    items(lang: $languageCode, itemId: $id) {
        title
        description
        id
        titleShort
        typeName
        icons {
            default
        }
        type {
            name
            title
        }
    }
}"#;

pub async fn fetch_item(lang: &str, item_id: u64) -> anyhow::Result<ItemData> {
    let body = serde_json::json! {
        [{
            "query": QUERY_ITEMS,
            "variables": {
                "id": item_id,
                "languageCode": lang,
            }
        }]
    };

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(non_camel_case_types)]
    struct ItemType__InternalBase {
        data: ItemType__InternalData,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(non_camel_case_types)]
    struct ItemType__InternalData {
        items: Vec<ItemData>,
    }

    let resp = reqwest::Client::new()
        .post("https://vortex.worldofwarships.asia/api/graphql/glossary/")
        .json(&body)
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to fetch item data: {:?}", e))?;

    let content = resp
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to parse from json: {:?}", e))?;

    // dbg!(&content);

    let content: Vec<ItemType__InternalBase> = serde_json::from_value(content)?;

    let item = content
        .get(0)
        .and_then(|t| t.data.items.get(0))
        .ok_or(anyhow::anyhow!("Failed to get a single result"))?;

    Ok(item.clone())
}

#[cfg(test)]
#[tokio::test]
async fn test_fetch_item() -> anyhow::Result<()> {
    let content = fetch_item("zh-sg", 4208586672).await?;
    println!("{:#?}", content);

    Ok(())
}
