use std::error::Error;
use std::usize;

use indicatif::{ProgressBar, ProgressStyle};
use octocrab::models::{issues::Comment, issues::Issue};
use octocrab::Page;
use serde::{Deserialize, Serialize};

use crate::github::GitHub;

pub struct IssueIterator {
    number_of_pages: usize,
    current_page: Page<Issue>,
    page_index: usize,
    issue_on_page_index: usize,
    progress_bar: ProgressBar,
    github: GitHub,
}


impl IssueIterator {
    pub async fn new(github: GitHub) -> Result<Self, Box<dyn Error + Sync + Send>> {
        let number_of_pages = github.number_of_issue_pages().await?;
        let progress_bar = ProgressBar::new(number_of_pages as u64 * u64::from(super::ITEMS_PER_PAGE));
        println!("Downloading issues for the selected repository");
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{pos}/{len} [{bar:40.cyan/blue}] {percent}% ({elapsed}/{eta}): {msg:.gray}",
                )
                .expect("correct template")
                .progress_chars("=> "),
        );
        let current_page = github.fetch_issues_page(0).await?;
        Ok(Self {
            current_page,
            page_index: 0,
            issue_on_page_index: 0,
            number_of_pages,
            progress_bar,
            github,
        })
    }

    pub(crate) async fn next(&mut self) -> Result<Option<SearchableIssue>, Box<dyn Error + Sync + Send>> {
        let issue = if let Some(issue) = self.current_page.items.get(self.issue_on_page_index) {
            issue
        } else {
            //no issue on current page, need to get new page
            let cannot_fetch_more_pages = self.page_index == self.number_of_pages;
            if cannot_fetch_more_pages {
                self.progress_bar.finish();
                return Ok(None);
            }
            self.issue_on_page_index = 0;
            self.page_index += 1;
            let page_index = u32::try_from(self.page_index).expect("at 100 items per page u32::MAX can never be reached");
            self.current_page = self.github.fetch_issues_page(page_index).await?;
            self.current_page.items
                .get(self.issue_on_page_index)
                .expect("got a new page from github, expecting that every page has at least 1 issue")
        };
        self.issue_on_page_index += 1;

        self.progress_bar.inc(1);
        self.progress_bar.set_message(format!(
            "Issue #{number}: {title}",
            number = issue.number,
            title = issue.title
        ));
        let comments = self.github.fetch_comments_for(issue.number).await?;
        Ok(Some(SearchableIssue::from(issue.clone()).with_comments(comments)))
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchableIssue {
    id: u64,
    title: String,
    body: String,
    comments: Vec<SearchableComment>,
}

impl From<Issue> for SearchableIssue {
    fn from(value: Issue) -> Self {
        Self {
            title: value.title,
            body: value.body.unwrap_or_default(),
            id: *value.id,
            comments: vec![],
        }
    }
}

impl SearchableIssue {
    pub fn with_comments(self, comments: Vec<Comment>) -> Self {
        Self {
            comments: comments.into_iter().map(SearchableComment::from).collect(),
            ..self
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchableComment {
    author: String,
    content: String,
}

impl From<Comment> for SearchableComment {
    fn from(comment: Comment) -> Self {
        Self {
            author: comment.user.login,
            content: comment.body.unwrap_or_default(),
        }
    }
}
