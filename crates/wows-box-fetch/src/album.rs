use serde::{Deserialize, Serialize};

use crate::item::ItemIcon;

const ALBUM_QUERY: &str = r#"query CollectibleAlbum ($albumId: String, $languageCode: String) {
    collectibleAlbum(albumId:$albumId, lang: $languageCode) {
        id
        title
        description
        icons {
            small
            large
            default
        }
    }
}"#;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumData {
    #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
    pub id: u64,
    pub icons: ItemIcon,
    pub title: String,
    #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_default_from_null")]
    pub description: String,
}

impl AlbumData {
    pub fn into_standard(self) -> wows_box::item::AlbumData {
        wows_box::item::AlbumData {
            id: self.id,
            name: self.title,
            icon: self.icons.into_default(),
        }
    }
}

pub async fn fetch_album(lang: &str, album_id: u64) -> anyhow::Result<AlbumData> {
    let body = serde_json::json! {[{
        "query": ALBUM_QUERY,
        "variables": {
            "albumId": album_id,
            "languageCode": lang,
        }
    }]};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(non_camel_case_types)]
    struct AlbumData__InternalBase {
        data: AlbumData__InternalData,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(non_camel_case_types)]
    struct AlbumData__InternalData {
        collectible_album: Vec<AlbumData>,
    }

    let resp = reqwest::Client::new()
        .post("https://vortex.worldofwarships.asia/api/graphql/glossary/")
        .json(&body)
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to fetch album data: {:?}", e))?;

    let content = resp
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to parse from json: {:?}", e))?;

    // dbg!(&content);

    let content: Vec<AlbumData__InternalBase> = serde_json::from_value(content)?;

    let item = content
        .get(0)
        .and_then(|t| t.data.collectible_album.get(0))
        .ok_or(anyhow::anyhow!("Failed to get a single result"))?;

    Ok(item.clone())
}

#[cfg(test)]
#[tokio::test]
async fn test_fetch_item() -> anyhow::Result<()> {
    let content = fetch_album("zh-sg", 4266630064).await?;
    println!("{:#?}", content);

    Ok(())
}
