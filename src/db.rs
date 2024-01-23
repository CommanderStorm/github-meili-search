use std::error::Error;

use chrono::{DateTime, Utc};
use sqlx::query_as;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::SqlitePool;
use sqlx::types::chrono::NaiveDateTime;

pub struct Db {
    pool: SqlitePool,
}

impl Db {
    pub async fn new(filename: &str) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let options = SqliteConnectOptions::new()
            .filename(filename)
            .create_if_missing(true);
        let pool = SqlitePool::connect_with(options).await?;
        sqlx::migrate!("./migrations").run(&pool).await?;
        Ok(Self { pool })
    }
    pub async fn store(&self, id: i64, hash: i64, last_update_at: DateTime<Utc>) -> Result<(), Box<dyn Error + Send + Sync>> {
        sqlx::query!(
            r#"INSERT INTO issue_log(id, hash, last_update_at)
            VALUES ($1, $2, $3)
            ON CONFLICT DO UPDATE SET hash=$2, last_update_at=$3"#,
            id, hash, last_update_at)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
    pub async fn last_change_at(&self) -> Result<NaiveDateTime, Box<dyn Error + Send + Sync>> {
        let res = query_as!(
            LastChangeResult,
            "SELECT MAX(last_update_at) as last_change_at FROM issue_log"
        )
            .fetch_optional(&self.pool)
            .await?;
        match res {
            None => Ok(NaiveDateTime::default()),
            Some(res) => Ok(res.last_change_at.unwrap_or(NaiveDateTime::default())),
        }
    }
}

struct LastChangeResult {
    last_change_at: Option<NaiveDateTime>,
}
