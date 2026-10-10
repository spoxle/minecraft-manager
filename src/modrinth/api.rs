use super::super::models;

const API: &str = "https://api.modrinth.com/v2";

pub struct ModrinthClient {
    client: reqwest::Client,
}

impl ModrinthClient {
    pub fn new() -> Result<Self, reqwest::Error> {
        let client = reqwest::Client::builder()
            .user_agent("minecraft-manager/0.1.0 (me@spoxle.com)")
            .build()?;

        Ok(Self { client })
    }

    pub async fn search_projects(
        &self,
        query: &str,
        facets: &[Vec<&str>],
        index: &str,
        offset: &str,
        limit: &str,
    ) -> Result<models::projects::Root, reqwest::Error> {
        let url = format!("{API}/search");

        let params = [
            ("query", query),
            ("facets", &serde_json::json!(facets).to_string()),
            ("index", &index),
            ("offset", &offset),
            ("limit", &limit),
        ];

        let projects: models::projects::Root = self
            .client
            .get(url)
            .query(&params)
            .send()
            .await?
            .json::<models::projects::Root>()
            .await?;

        Ok(projects)
    }

    // pub async fn fetch_project(id: &str) -> Result<models::project::Root, reqwest::Error> {}
}
