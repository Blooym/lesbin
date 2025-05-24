use anyhow::Result;
use sqlx::{
    SqlitePool, migrate,
    sqlite::{SqliteAutoVacuum, SqliteConnectOptions, SqliteJournalMode},
};
use std::str::FromStr;

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(connect_string: &str) -> Result<Self> {
        let pool = SqlitePool::connect_with(
            SqliteConnectOptions::from_str(connect_string)?
                .journal_mode(SqliteJournalMode::Wal)
                .auto_vacuum(SqliteAutoVacuum::Full)
                .optimize_on_close(true, None)
                .create_if_missing(true),
        )
        .await?;
        migrate!().run(&pool).await?;
        Ok(Self { pool })
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }
}
