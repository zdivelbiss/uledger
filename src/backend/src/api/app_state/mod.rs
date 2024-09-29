use axum_extra::extract::cookie;
use base64::Engine;
use session_state::SessionState;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use user_state::UserState;

pub mod session_state;
pub mod user_state;

#[derive(Clone)]
pub struct AppState {
    cookie_key: cookie::Key,
    user_state: UserState,
    session_state: SessionState,
}

impl AppState {
    pub async fn create() -> Self {
        use crate::config::cfg;
        use base64::engine::general_purpose::STANDARD;

        let key_bytes = STANDARD
            .decode(cfg().apikey.cookies.as_str())
            .expect("failed to decode BASE64 cookies API key");
        let cookie_key =
            cookie::Key::try_from(key_bytes.as_slice()).expect("cookies API key is invalid");
        drop(key_bytes);

        let db_pool = PgPoolOptions::new()
            .max_connections(cfg().database.pool.size)
            .acquire_timeout(Duration::from_secs(3600))
            .connect(cfg().database.url.as_str())
            .await
            .expect("could not initialize DB connection pool");

        let user_state = UserState::new(db_pool.clone());
        let session_state = SessionState::connect()
            .await
            .expect("RESP connection issue");

        Self {
            cookie_key,
            user_state,
            session_state,
        }
    }

    pub fn cookie_key(&self) -> &cookie::Key {
        &self.cookie_key
    }

    pub fn user_state(&self) -> &UserState {
        &self.user_state
    }

    pub fn session_state(&self) -> &SessionState {
        &self.session_state
    }
}

impl axum::extract::FromRef<AppState> for cookie::Key {
    fn from_ref(state: &AppState) -> Self {
        state.cookie_key.clone()
    }
}
