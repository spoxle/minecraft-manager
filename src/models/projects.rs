use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {
    pub hits: Vec<Hit>,
    pub offset: i64,
    pub limit: i64,
    pub total_hits: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hit {
    pub project_id: String,
    pub project_type: String,
    pub all_project_types: Vec<String>,
    pub slug: String,
    pub author: String,
    pub author_id: String,
    pub organization: Option<String>,
    pub organization_id: Option<String>,
    pub title: String,
    pub description: String,
    pub categories: Vec<String>,
    pub display_categories: Vec<String>,
    pub versions: Vec<String>,
    pub downloads: i64,
    pub follows: i64,
    pub icon_url: String,
    pub date_created: String,
    pub date_modified: String,
    pub latest_version: String,
    pub license: String,
    pub client_side: String,
    pub server_side: String,
    pub environment: Vec<String>,
    pub disclosure_types: Vec<String>,
    pub gallery: Vec<String>,
    pub featured_gallery: Option<String>,
    pub color: Option<i64>,
}
