use crate::{
    server::{UserSession, api::crud_router, internal_error, serialize_json_response},
    state::{
        AppState,
        ledger::account::{AccountKind, AccountLedger},
    },
};
use axum::{
    Router,
    extract::{Form, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

pub fn router() -> Router<AppState> {
    crud_router(_read_all, _create, _read, _update, _delete)
}

#[derive(Debug, Deserialize)]
struct Info {
    name: String,
    kind: AccountKind,
    description: Option<String>,
}

async fn _read_all(
    user_session: UserSession,
    State(ledger): State<AccountLedger>,
) -> impl IntoResponse {
    match ledger.read_all(user_session.id()).await {
        Ok(records) => serialize_json_response(&records).into_response(),

        Err(error) => internal_error(error).into_response(),
    }
}

async fn _create(
    user_session: UserSession,
    State(ledger): State<AccountLedger>,
    Form(Info {
        name,
        kind,
        description,
    }): Form<Info>,
) -> impl IntoResponse {
    use crate::state::ledger::account::create::Error;

    match ledger
        .create(
            user_session.id(),
            name.as_str(),
            kind,
            description.as_deref(),
        )
        .await
    {
        Ok(record) => serialize_json_response(&record).into_response(),

        Err(Error::Duplicate) => (StatusCode::CONFLICT, "Account already exists.").into_response(),
        Err(Error::NameLength) => {
            (StatusCode::BAD_REQUEST, "Account name is too long.").into_response()
        }
        Err(Error::DescriptionLength) => {
            (StatusCode::BAD_REQUEST, "Account description is too long.").into_response()
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
        Ok(record) => serialize_json_response(&record).into_response(),

        Err(Error::NotFound) => (StatusCode::NOT_FOUND, "Account was not found.").into_response(),
        Err(error) => internal_error(error).into_response(),
    }
}

async fn _update(
    user_session: UserSession,
    State(ledger): State<AccountLedger>,
    Path(id): Path<Uuid>,
    Form(Info {
        name,
        kind,
        description,
    }): Form<Info>,
) -> impl IntoResponse {
    use crate::state::ledger::account::update::Error;

    match ledger
        .update(
            user_session.id(),
            id,
            name.as_str(),
            kind,
            description.as_deref(),
        )
        .await
    {
        Ok(record) => serialize_json_response(&record).into_response(),

        Err(Error::NotFound) => (StatusCode::NOT_FOUND, "Account was not found.").into_response(),
        Err(Error::Duplicate) => (StatusCode::CONFLICT, "Account already exists.").into_response(),
        Err(Error::NameLength) => {
            (StatusCode::BAD_REQUEST, "Account name is too long.").into_response()
        }
        Err(Error::DescriptionLength) => {
            (StatusCode::BAD_REQUEST, "Account description is too long.").into_response()
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
        Ok(_) => (StatusCode::OK, "Account has been deleted.").into_response(),

        Err(Error::NotFound) => (StatusCode::NOT_FOUND, "Account was not found.").into_response(),
        Err(error) => internal_error(error).into_response(),
    }
}
