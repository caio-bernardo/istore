use crate::error::Error;
use axum::extract::FromRef;
use sqlx::{Pool, Sqlite, SqlitePool};

mod base;
pub mod task;
pub mod user;

type Db = Pool<Sqlite>;

#[derive(Debug, Clone, FromRef)]
pub struct ModelManager {
    pub db: Db,
}

impl ModelManager {
    pub async fn new() -> Self {
        Self::new_db_pool().await.unwrap()
    }

    async fn new_db_pool() -> Result<Self, Error> {
        let db = SqlitePool::connect("sqlite://database/database.db").await?;
        Ok(ModelManager { db })
    }

    fn db(&self) -> &Db {
        &self.db
    }
}
