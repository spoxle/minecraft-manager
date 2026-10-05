use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub hits: Vec<Hit>,
    pub offset: i64,
    pub limit: i64,
    #[serde(rename = "total_hits")]
    pub total_hits: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hit {
    #[serde(rename = "project_id")]
    pub project_id: String,
    #[serde(rename = "project_type")]
    pub project_type: String,
    #[serde(rename = "all_project_types")]
    pub all_project_types: Vec<String>,
    pub slug: String,
    pub author: String,
    #[serde(rename = "author_id")]
    pub author_id: String,
    pub organization: Option<String>,
    #[serde(rename = "organization_id")]
    pub organization_id: Option<String>,
    pub title: String,
    pub description: String,
    pub categories: Vec<String>,
    #[serde(rename = "display_categories")]
    pub display_categories: Vec<String>,
    pub versions: Vec<String>,
    pub downloads: i64,
    pub follows: i64,
    #[serde(rename = "icon_url")]
    pub icon_url: String,
    #[serde(rename = "date_created")]
    pub date_created: String,
    #[serde(rename = "date_modified")]
    pub date_modified: String,
    #[serde(rename = "latest_version")]
    pub latest_version: String,
    pub license: String,
    #[serde(rename = "client_side")]
    pub client_side: String,
    #[serde(rename = "server_side")]
    pub server_side: String,
    pub environment: Vec<String>,
    #[serde(rename = "disclosure_types")]
    pub disclosure_types: Vec<String>,
    pub gallery: Vec<String>,
    #[serde(rename = "featured_gallery")]
    pub featured_gallery: Option<String>,
    pub color: Option<i64>,
}
