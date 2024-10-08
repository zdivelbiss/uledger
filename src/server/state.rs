use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::time::Duration;

#[derive(Clone)]
pub struct AppState {
    db: Pool<Postgres>,
}

impl AppState {
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

    pub fn db(&self) -> &Pool<Postgres> {
        &self.db
    }
}

pub async fn get_user_id(session: &tower_sessions::Session) -> uuid::Uuid {
    match session.get("user_id").await {
        Ok(Some(user_id)) => user_id,
        _ => panic!("user is not authenticated"),
    }
}
