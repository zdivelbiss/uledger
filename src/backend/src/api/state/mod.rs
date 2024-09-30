use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use user::UserState;

pub mod user;

#[derive(Clone)]
pub struct AppState {
    user_state: UserState,
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

        Self {
            user_state: UserState::new(db_pool.clone()),
        }
    }

    pub fn user_state(&self) -> &UserState {
        &self.user_state
    }
}
