use sessions::SessionState;
use tokio::sync::{OnceCell, SetError};
use users::UserState;
use verifications::VerificationState;

pub mod sessions;
pub mod users;
pub mod verifications;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("state already initializing")]
    StateInitializing,

    #[error("state already initialized")]
    StateInitialized,

    #[error("session error")]
    Sessions(#[from] sessions::Error),

    #[error("verifications error")]
    Verifications(#[from] verifications::Error),

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
        .max_connections(cfg().database_pool_size())
        .connect(cfg().database_url())
        .await?;
    MIGRATOR.run(&db_pool).await?;

    let user_state = UserState::new(db_pool.clone());
    let session_state = SessionState::connect().await?;
    let verification_state = VerificationState::connect().await?;

    let state = AppState {
        user_state,
        session_state,
        verification_state,
    };

    STATE.set(state)?;

    Ok(())
}

pub fn get() -> AppState {
    STATE.get().expect("state is uninitialized").clone()
}

#[derive(Clone)]
pub struct AppState {
    user_state: UserState,
    session_state: SessionState,
    verification_state: VerificationState,
}
