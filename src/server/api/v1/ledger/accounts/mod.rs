use crate::server::{responses::internal_error, state::AppState};
use axum::{
    extract::State,
    response::{IntoResponse, Response},
    routing::{delete, get, patch, post},
    Json,
};
use chrono::{DateTime, Utc};
use tower_sessions::Session;

mod create;
mod delete;
mod read;
mod update;

pub fn router() -> axum::Router<AppState> {
    axum::Router::new().route("/", post(create::create))
    // .route("/", get(read::read))
    // .route("/", patch(update::update))
    // .route("/", delete(delete::delete))
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Database(#[from] sqlx::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::Database(error) => internal_error(error).into_response(),
        }
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, FromPrimitive, Serialize, Deserialize,
)]
#[serde(rename_all = "UPPERCASE")]
#[repr(i16)]
enum Kind {
    #[default]
    Equity = 0,
    Asset = 1,
    Liability = 2,
    Income = 3,
    Expense = 4,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct Account {
    created: DateTime<Utc>,
    kind: Kind,
    name: String,
    description: Option<String>,
}

async fn get_accounts(
    session: Session,
    state: State<AppState>,
) -> Result<Json<Vec<Account>>, Error> {
    // let user_id = crate::get_user_id(&session).await;

    // let accounts = query_as!(
    //     Account,
    //     "
    //     SELECT created, kind, name, description
    //         FROM ledger.accounts
    //         WHERE user_id = $1
    //     ;
    //     ",
    //     user_id
    // )
    // .fetch_all(state.db())
    // .await?;

    // Ok(accounts.into())

    todo!()
}
