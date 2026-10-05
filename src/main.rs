mod models;
mod modrinth;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .user_agent("minecraft-manager/0.1.0 (me@spoxle.com)")
        .build()?;

    let facets = vec![vec!["project_type:resourcepack".to_string()]];

    let projects = match modrinth::api::search_projects(&client, &facets, "downloads", "").await {
        Ok(projects) => projects
            .hits
            .into_iter()
            .map(|p| p.title)
            .collect::<Vec<String>>(),
        Err(e) => {
            eprintln!("{}", e);
            Vec::new()
        }
    };

    println!("{:#?}", projects);

    Ok(())
}
