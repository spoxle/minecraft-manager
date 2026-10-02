use super::super::models;

const API: &str = "https://api.modrinth.com/v2";

pub async fn search_projects(
    client: &reqwest::Client,
) -> Result<models::projects::Root, reqwest::Error> {
    let url = format!("{API}/search");

    let projects: models::projects::Root = client
        .get(url)
        .send()
        .await?
        .json::<models::projects::Root>()
        .await?;

    Ok(projects)
}
