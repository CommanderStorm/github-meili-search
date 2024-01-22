use std::error::Error;
use std::io;
use std::time::Duration;

use meilisearch_sdk::{Client, Settings, Task};
use serde::Serialize;

const TIMEOUT: Option<Duration> = Some(Duration::from_secs(20));
const POLLING_RATE: Option<Duration> = Some(Duration::from_millis(50));

pub(crate) struct Meilisearch {
    client: Client,
}

impl Meilisearch {
    fn setttings() -> Settings {
        Settings::new()
            .with_ranking_rules([
                "words",
                "typo",
                "rank:desc",
                "proximity",
                "attribute",
                "sort",
                "exactness",
            ])
            .with_searchable_attributes(["id", "title", "body", "comments"])
    }
    pub async fn new(host: impl Into<String>, api_key: Option<impl Into<String>>) -> Result<Self, Box<dyn Error + Sync + Send>> {
        let client = meilisearch_sdk::Client::new(host, api_key);
        client
            .create_index("issues", Some("id"))
            .await?
            .wait_for_completion(&client, POLLING_RATE, TIMEOUT)
            .await?;
        let issues = client.index("issues");

        let settings = Meilisearch::setttings();
        let res = issues
            .set_settings(&settings)
            .await?
            .wait_for_completion(&client, POLLING_RATE, TIMEOUT)
            .await?;
        match res {
            Task::Failed { content } => {
                Err(io::Error::other(format!("Failed to setup Meilisearch: {content:#?}")).into())
            }
            _ => Ok(Self { client }),
        }
    }
    pub async fn store<T: Serialize>(&self, issue: T) -> Result<(), Box<dyn Error + Sync + Send>> {
        let issues = self.client.index("issues");
        let to_insert = vec![issue];
        let res = issues
            .add_documents(&to_insert, Some("ms_id"))
            .await?
            .wait_for_completion(&self.client, POLLING_RATE, TIMEOUT)
            .await?;
        match res {
            Task::Failed { content } => Err(io::Error::other(format!(
                "Failed to add documents to Meilisearch: {content:#?}"
            ))
                .into()),
            _ => Ok(()),
        }
    }
}
