use crate::{
    api::crud_router,
    internal_error,
    state::{App, ledger::account::AccountLedger},
    user_session::UserSession,
};
use axum::{
    Router,
    extract::{Form, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

pub fn router() -> Router<App> {
    crud_router(_read_all, _create, _read, _update, _delete)
}

#[derive(Debug, Deserialize)]
struct Info {
    name: String,
    description: Option<String>,
}

async fn _read_all(
    user_session: UserSession,
    State(ledger): State<AccountLedger>,
) -> impl IntoResponse {
    match ledger.read_all(user_session.id()).await {
        Ok(records) => todo!(),

        Err(error) => internal_error(error).into_response(),
    }
}

async fn _create(
    user_session: UserSession,
    State(ledger): State<AccountLedger>,
    Form(Info { name, description }): Form<Info>,
) -> impl IntoResponse {
    use crate::state::ledger::account::create::Error;

    match ledger
        .create(user_session.id(), name.as_str(), description.as_deref())
        .await
    {
        Ok(record) => todo!(),

        Err(Error::Duplicate) => (StatusCode::CONFLICT, "account already exists").into_response(),
        Err(Error::NameLength) => {
            (StatusCode::BAD_REQUEST, "account name too long").into_response()
        }
        Err(Error::DescriptionLength) => {
            (StatusCode::BAD_REQUEST, "account description too long").into_response()
        }
        Err(error) => internal_error(error).into_response(),
    }
}

async fn _read(
    user_session: UserSession,
    State(ledger): State<AccountLedger>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    use crate::state::ledger::account::read::Error;

    match ledger.read(user_session.id(), id).await {
        Ok(record) => todo!(),

        Err(Error::NotFound) => (StatusCode::NOT_FOUND, "account not found").into_response(),
        Err(error) => internal_error(error).into_response(),
    }
}

async fn _update(
    user_session: UserSession,
    State(ledger): State<AccountLedger>,
    Path(id): Path<Uuid>,
    Form(Info { name, description }): Form<Info>,
) -> impl IntoResponse {
    use crate::state::ledger::account::update::Error;

    match ledger
        .update(user_session.id(), id, name.as_str(), description.as_deref())
        .await
    {
        Ok(record) => todo!(),

        Err(Error::NotFound) => (StatusCode::NOT_FOUND, "account not found").into_response(),
        Err(Error::Duplicate) => (StatusCode::CONFLICT, "account already exists").into_response(),
        Err(Error::NameLength) => {
            (StatusCode::BAD_REQUEST, "account name too long").into_response()
        }
        Err(Error::DescriptionLength) => {
            (StatusCode::BAD_REQUEST, "account description too long").into_response()
        }
        Err(error) => internal_error(error).into_response(),
    }
}

async fn _delete(
    user_session: UserSession,
    State(ledger): State<AccountLedger>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    use crate::state::ledger::account::delete::Error;

    match ledger.delete(user_session.id(), id).await {
        Ok(_) => todo!(),

        Err(Error::NotFound) => (StatusCode::NOT_FOUND, "account not found").into_response(),
        Err(error) => internal_error(error).into_response(),
    }
}
