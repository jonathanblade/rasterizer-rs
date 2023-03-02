use sqlx::migrate;
use sqlx::pool::PoolOptions;
use std::time::Duration;
use url::Url;

use crate::types::DbPool;

pub struct RasterizerDatabase {
    pub pool: DbPool,
}

impl RasterizerDatabase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn create_pool(db_url: Url) -> DbPool {
        let pool = PoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(5))
            .connect(db_url.as_str())
            .await
            .expect("Failed to create database connection pool");
        migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Failed to run database migrations");
        pool
    }
}
