use crate::config::CONFIG;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::time::Duration;

pub mod ledger;
pub mod user;

#[derive(Clone)]
pub struct AppState {
    db: Pool<Postgres>,
}

impl AppState {
    pub async fn create() -> Self {
        let db_pool = PgPoolOptions::new()
            .max_connections(CONFIG.database.pool.size)
            .acquire_timeout(Duration::from_secs(3600))
            .connect(CONFIG.database.url.as_str())
            .await
            .expect("could not initialize DB connection pool");

        Self { db: db_pool }
    }
}
