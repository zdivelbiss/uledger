use std::time::Duration;

use session_state::SessionState;
use tokio::sync::{OnceCell, SetError};
use user_state::UserState;
use verify_state::VerifyState;

pub mod session_state;
pub mod user_state;
pub mod verify_state;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("state already initializing")]
    StateInitializing,

    #[error("state already initialized")]
    StateInitialized,

    #[error("session error")]
    Sessions(#[from] session_state::Error),

    #[error("verifications error")]
    Verifications(#[from] verify_state::Error),

    #[error("sqlx error")]
    Sqlx(#[from] sqlx::Error),

    #[error("migration error")]
    Migration(#[from] sqlx::migrate::MigrateError),

    #[error("unknown error")]
    Unknown(Box<dyn std::error::Error>),
}

impl From<SetError<AppState>> for Error {
    fn from(value: SetError<AppState>) -> Self {
        match value {
            SetError::AlreadyInitializedError(_) => Self::StateInitialized,
            SetError::InitializingError(_) => Self::StateInitializing,
        }
    }
}

type Result<T> = std::result::Result<T, Error>;

static STATE: OnceCell<AppState> = OnceCell::const_new();

pub async fn init() -> Result<()> {
    use crate::cfg;

    static MIGRATOR: sqlx::migrate::Migrator = migrate!();

    let db_pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(cfg().database.pool.size)
        .acquire_timeout(Duration::from_secs(3600))
        .connect(cfg().database.url.as_str())
        .await?;
    MIGRATOR.run(&db_pool).await?;

    let state = AppState(
        UserState::new(db_pool.clone()),
        SessionState::connect().await?,
        VerifyState::connect().await?,
    );

    STATE.set(state)?;

    Ok(())
}

pub fn get() -> AppState {
    STATE.get().expect("state is uninitialized").clone()
}

#[derive(Clone)]
pub struct AppState(UserState, SessionState, VerifyState);
