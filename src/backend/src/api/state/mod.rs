use anyhow::Result;
use sessions::SessionState;
use tokio::sync::{OnceCell, SetError};
use users::UserState;
use verification::VerificationState;

mod sessions;
mod users;
mod verification;

static STATE: OnceCell<State> = OnceCell::const_new();

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

    let state = State {
        user_state,
        session_state,
        verification_state,
    };

    match STATE.set(state) {
        Ok(_) => Ok(()),

        Err(SetError::<State>::InitializingError(_)) => bail!("state is already being initialized"),
        Err(SetError::<State>::AlreadyInitializedError(_)) => {
            bail!("state has already been initialized")
        }
    }
}

pub fn get() -> State {
    STATE.get().expect("state is uninitialized").clone()
}

#[derive(Clone)]
pub struct State {
    user_state: UserState,
    session_state: SessionState,
    verification_state: VerificationState,
}
