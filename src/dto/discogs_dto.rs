use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscogsRoot {
    pub pagination: Pagination,
    pub results: Vec<DiscogsRecord>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    pub page: i64,
    pub pages: i64,
    #[serde(rename = "per_page")]
    pub per_page: i64,
    pub items: i64,
    pub urls: Urls,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Urls {
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscogsRecord {
    pub country: String,
    pub year: Option<String>,
    pub format: Vec<String>,
    pub label: Vec<String>,
    #[serde(rename = "type")]
    pub type_field: String,
    pub genre: Vec<String>,
    pub style: Vec<String>,
    pub id: i64,
    pub barcode: Vec<String>,
    #[serde(rename = "user_data")]
    pub user_data: UserData,
    #[serde(rename = "master_id")]
    pub master_id: i64,
    #[serde(rename = "master_url")]
    pub master_url: String,
    pub uri: String,
    pub catno: String,
    pub title: String,
    pub thumb: String,
    #[serde(rename = "cover_image")]
    pub cover_image: String,
    #[serde(rename = "resource_url")]
    pub resource_url: String,
    pub community: Community,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserData {
    #[serde(rename = "in_wantlist")]
    pub in_wantlist: bool,
    #[serde(rename = "in_collection")]
    pub in_collection: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Community {
    pub want: i64,
    pub have: i64,
}
