use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

use sqlx::sqlite::SqliteConnectOptions;
use sqlx::SqlitePool;
use sqlx::types::chrono::DateTime;

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
        let now_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).expect("time does not move backwards");
        let now = DateTime::from_timestamp(now_timestamp.as_secs() as i64, now_timestamp.subsec_nanos()).expect("current time is a valid timestamp");
        sqlx::query!(r#"INSERT INTO issue_log(id, hash, last_update_at)
VALUES ($1, $2, $3)
ON CONFLICT DO UPDATE SET hash=$2, last_update_at=$3"#, id, hash, now).execute(&self.pool).await?;
        Ok(())
    }
}