use crate::config;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::time::Duration;

pub mod ledger;
pub mod user;

#[derive(Clone)]
pub struct App {
    db: Pool<Postgres>,
}

impl App {
    pub async fn create() -> Self {
        let db_pool = PgPoolOptions::new()
            .max_connections(config::get().database.pool.size)
            .acquire_timeout(Duration::from_secs(3600))
            .connect(config::get().database.url.as_str())
            .await
            .expect("could not initialize DB connection pool");

        Self { db: db_pool }
    }
}
