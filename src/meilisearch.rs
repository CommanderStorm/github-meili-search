use std::error::Error;
use std::io;
use std::time::Duration;

use meilisearch_sdk::{Client, Settings, Task};
use serde::{Deserialize, Serialize};

const TIMEOUT: Option<Duration> = Some(Duration::from_secs(20));
const POLLING_RATE: Option<Duration> = Some(Duration::from_millis(50));

pub struct Meilisearch {
    client: Client,
}

#[derive(Debug, Serialize, Deserialize)]
struct Embedder {
    source: String,
    model: String,
    #[serde(rename = "documentTemplate")]
    document_template: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Embedders {
    default: Embedder,
}

#[derive(Debug, Serialize, Deserialize)]
struct EmbedderSettings {
    embedders: Embedders,
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
    pub async fn new(
        host: String,
        api_key: Option<String>,
    ) -> Result<Self, Box<dyn Error + Sync + Send>> {
        let client = Client::new(host.clone(), api_key);
        meilisearch_sdk::ExperimentalFeatures::new(&client)
            .set_vector_store(true)
            .update()
            .await?;

        // create a fresh index
        let _ = client.delete_index("issues").await;
        if let Task::Failed { content } = client
            .create_index("issues", Some("id"))
            .await?
            .wait_for_completion(&client, POLLING_RATE, TIMEOUT)
            .await?
        {
            return Err(io::Error::other(format!("could not create index: {content:#?}")).into());
        }
        let issues = client.index("issues");

        let req_client = reqwest::Client::new();
        let embedding_settings = EmbedderSettings {
            embedders: Embedders {
                default: Embedder {
                    source: "huggingFace".to_string(),
                    model: "BAAI/bge-base-en-v1.5".to_string(),
                    document_template: "A github issue titled '{{doc.title}}' whose description starts with {{doc.body|truncatewords: 20}}".to_string(),
                }
            }
        };
        let url = format!("{host}/indexes/issues/settings");
        let res = req_client
            .patch(url)
            .json(&embedding_settings)
            .send()
            .await?;
        if res.status() != 202 {
            return Err(io::Error::other(format!(
                "Failed to enable embedding because {code}: {text}",
                code = res.status(),
                text = res.text().await?
            ))
            .into());
        }

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
    pub async fn store<T: Serialize>(
        &self,
        documents: &[T],
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let issues = self.client.index("issues");
        let res = issues
            .add_documents(documents, Some("id"))
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
