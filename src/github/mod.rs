use std::error::Error;
use std::time::Duration;

use octocrab::models::issues::{Comment, Issue};
use octocrab::params::issues::Sort;
use octocrab::params::Direction;
use octocrab::{Octocrab, Page};
use sqlx::types::chrono::{DateTime, NaiveDateTime, Utc};
use tokio::time::sleep;

use crate::github::issue_iterator::IssueIterator;

mod issue_iterator;

pub struct GitHub {
    client: Octocrab,
    owner: String,
    repo: String,
    last_change_at: DateTime<Utc>,
}

const TIME_BETWEEN_GITHUB_REQUESTS: Duration = Duration::from_millis(200);

const ITEMS_PER_PAGE: u8 = 100;

impl GitHub {
    pub fn new(
        pat: &str,
        owner: &str,
        repo: &str,
        last_change_at: &NaiveDateTime,
    ) -> Result<GitHub, octocrab::Error> {
        Ok(Self {
            client: Octocrab::builder()
                .personal_token(pat.to_string())
                .build()?,
            owner: owner.to_string(),
            repo: repo.to_string(),
            last_change_at: last_change_at.clone().and_utc(),
        })
    }
    pub async fn iter_issues(self) -> Result<IssueIterator, Box<dyn Error + Sync + Send>> {
        IssueIterator::new(self).await
    }
    async fn number_of_issue_pages(&self) -> Result<usize, Box<dyn Error + Sync + Send>> {
        let number_of_pages = self
            .fetch_issues_page(0)
            .await?
            .number_of_pages()
            .unwrap_or(0);

        Ok(number_of_pages as usize)
    }
    async fn fetch_issues_page(
        &self,
        index: u32,
    ) -> Result<Page<Issue>, Box<dyn Error + Sync + Send>> {
        sleep(TIME_BETWEEN_GITHUB_REQUESTS).await;
        let page = self
            .client
            .issues(&self.owner, &self.repo)
            .list()
            .since(self.last_change_at)
            .sort(Sort::Updated)
            .direction(Direction::Ascending)
            .state(octocrab::params::State::All)
            .per_page(ITEMS_PER_PAGE) // Adjust the per_page value as needed
            .page(index)
            .send()
            .await?;
        Ok(page)
    }
    async fn fetch_comments_for(
        &self,
        issue_number: u64,
    ) -> Result<Vec<Comment>, Box<dyn Error + Sync + Send>> {
        sleep(TIME_BETWEEN_GITHUB_REQUESTS).await;
        let comments = self
            .client
            .issues(&self.owner, &self.repo)
            .list_comments(issue_number)
            .per_page(ITEMS_PER_PAGE)
            .page(0_u32)
            .send()
            .await?;
        Ok(self.client.all_pages(comments).await?)
    }
}
