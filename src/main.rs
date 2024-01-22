use std::error::Error;
use clap::Parser;

use crate::github::GitHub;
use crate::meilisearch::Meilisearch;

mod github;
mod meilisearch;
mod db;

#[derive(Parser, Debug)]
pub struct Opts {
    /// The owner of the GitHub Repository
    #[clap(long, env, required = true)]
    owner: String,

    /// The GitHub Repository name
    #[clap(long, env, required = true)]
    repo: String,

    /// The GitHub Personal Access Token (PAT) required for authentication
    #[clap(long, env, required = true)]
    github_pat: String,

    /// The URL for the MeiliSearch instance
    #[clap(long, env, required = false)]
    #[clap(default_value = "http://localhost:7700")]
    meili_url: String,

    /// The MeiliSearch master key
    #[clap(long, env, required = false)]
    #[clap(default_value = None)]
    meili_master_key: Option<String>,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    let opt = Opts::parse();
    let ms_client = Meilisearch::new(opt.meili_url, opt.meili_master_key).await?;
    let db=db::Db::new("download_log.sqlite").await?;
    let github = GitHub::new(&opt.github_pat, &opt.owner, &opt.repo)?;
    let mut issues = github.iter_issues().await?;
    while let Some(issue) = issues.next().await? {
        ms_client.store(&[issue]).await?;
    }
    Ok(())
}
