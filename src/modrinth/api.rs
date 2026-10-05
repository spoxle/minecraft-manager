use super::super::models;

const API: &str = "https://api.modrinth.com/v2";

pub async fn search_projects(
    client: &reqwest::Client,
    facets: &[Vec<String>],
    index: &str,
    query: &str,
) -> Result<models::projects::Root, reqwest::Error> {
    let url = format!("{API}/search");

    let params = [
        ("query", query),
        ("facets", &serde_json::json!(facets).to_string()),
        ("index", &index),
        ("limit", "100"),
    ];

    let projects: models::projects::Root = client
        .get(url)
        .query(&params)
        .send()
        .await?
        .json::<models::projects::Root>()
        .await?;

    Ok(projects)
}
