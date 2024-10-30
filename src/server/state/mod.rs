use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::time::Duration;

pub mod ledger;
pub mod user;

#[derive(Clone)]
pub struct App {
    db: Pool<Postgres>,
}

impl App {
    pub async fn create() -> Self {
        use crate::config::cfg;

        let db_pool = PgPoolOptions::new()
            .max_connections(cfg().database.pool.size)
            .acquire_timeout(Duration::from_secs(3600))
            .connect(cfg().database.url.as_str())
            .await
            .expect("could not initialize DB connection pool");

        Self { db: db_pool }
    }
}
