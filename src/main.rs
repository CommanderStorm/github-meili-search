use std::error::Error;

use crate::github::GitHub;
use crate::meilisearch::Meilisearch;

mod github;
mod meilisearch;

const OWNER: &str = "TUM-Dev";
const REPO: &str = "NavigaTUM";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    let ms_client = Meilisearch::new(
        std::env::var("MEILI_URL").unwrap_or("http://localhost:7700".into()),
        std::env::var("MEILI_MASTER_KEY").ok(),
    ).await?;
    let pat = std::env::var("GITHUB_PAT").expect("GITHUB_PAT is required to run this app");
    let github = GitHub::setup(&pat, OWNER, REPO)?;
    let mut issues = github.iter_issues().await?;
    while let Some(issue) = issues.next().await? {
        ms_client.store(issue).await?;
    }
    Ok(())
}
