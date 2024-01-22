use std::error::Error;

use sqlx::query_as;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::SqlitePool;

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
    pub async fn store(&self, id: i64, hash: i64) -> Result<(), Box<dyn Error + Send + Sync>> {
        let now = chrono::Utc::now();
        sqlx::query!(
            r#"INSERT INTO issue_log(id, hash, last_update_at)
VALUES ($1, $2, $3)
ON CONFLICT DO UPDATE SET hash=$2, last_update_at=$3"#,
            id,
            hash,
            now
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}