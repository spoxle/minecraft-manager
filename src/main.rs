mod models;
mod modrinth;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .user_agent("minecraft-manager/0.1.0 (me@spoxle.com)")
        .build()?;

    let projects = modrinth::api::search_projects(&client).await;

    match projects {
        Ok(data) => {
            let slugs: Vec<String> = data.hits.into_iter().map(|p| p.slug).collect();

            println!("{:#?}", slugs)
        }
        Err(e) => eprintln!("{}", e),
    }

    Ok(())
}
