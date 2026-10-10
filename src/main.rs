mod models;
mod modrinth;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = modrinth::api::ModrinthClient::new()?;

    let facets = vec![
        // vec!["project_type:modpack", "project_type:resourcepack"],
        // vec!["versions:1.21.1", "versions:26.2"],
    ];

    let projects = client
        .search_projects("", &facets, "relevance", "0", "20")
        .await?;

    println!("{:#?}", projects);

    Ok(())
}
