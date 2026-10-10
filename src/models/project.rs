use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {
    pub client_side: String,
    pub server_side: String,
    pub game_versions: Vec<String>,
    pub environment: Vec<String>,
    pub id: String,
    pub slug: String,
    pub project_type: String,
    pub team: String,
    pub organization: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub body_url: String,
    pub published: String,
    pub updated: String,
    pub approved: String,
    pub queued: String,
    pub status: String,
    pub requested_status: String,
    pub moderator_message: ModeratorMessage,
    pub license: License,
    pub downloads: i64,
    pub followers: i64,
    pub categories: Vec<String>,
    pub additional_categories: Vec<String>,
    pub loaders: Vec<String>,
    pub versions: Vec<String>,
    pub icon_url: String,
    pub raw_icon_url: String,
    pub issues_url: String,
    pub source_url: String,
    pub wiki_url: String,
    pub discord_url: String,
    pub donation_urls: Vec<DonationUrl>,
    pub gallery: Vec<Gallery>,
    pub color: i64,
    pub thread_id: String,
    pub monetization_status: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct License {
    pub id: String,
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DonationUrl {
    pub id: String,
    pub platform: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Gallery {
    pub url: String,
    pub raw_url: String,
    pub featured: bool,
    pub title: String,
    pub description: String,
    pub created: String,
    pub ordering: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModeratorMessage {
    pub message: String,
    pub body: String,
}
