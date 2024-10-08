use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemData {
    pub name: String,
    #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_default_from_null")]
    pub short_name: String,
    pub id: u64,
    /// only `default` icon url.
    pub icon: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumData {
    pub name: String,
    pub id: u64,
    pub icon: String,
}

impl AlbumData {
    pub fn into_item(self) -> ItemData {
        ItemData {
            name: self.name,
            short_name: String::new(),
            id: self.id,
            icon: self.icon,
        }
    }
}
