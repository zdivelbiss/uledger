use std::time::Duration;

use session_state::SessionState;
use tokio::sync::{OnceCell, SetError};
use user_state::UserState;

pub mod session_state;
pub mod user_state;

#[derive(Clone)]
pub struct AppState(UserState, SessionState);

impl AppState {
    pub async fn create() -> Self {
        use crate::cfg;

        let db_pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(cfg().database.pool.size)
            .acquire_timeout(Duration::from_secs(3600))
            .connect(cfg().database.url.as_str())
            .await
            .expect("could not initialize DB connection pool");

        let user_state = UserState::new(db_pool.clone());
        let session_state = SessionState::connect()
            .await
            .expect("RESP connection issue");

        Self(user_state, session_state)
    }

    pub fn user_state(&self) -> &UserState {
        &self.0
    }

    pub fn session_state(&self) -> &SessionState {
        &self.1
    }
}
