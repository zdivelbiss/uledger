use crate::server::{
    htmx::IsHtmx,
    internal_error,
    state::{ledger::account::AccountLedger, App},
    UserSession,
};
use axum::{
    extract::{Form, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Router,
};
use lib::ledger::account::AccountKind;
use uuid::Uuid;

pub fn router() -> Router<App> {
    Router::new()
        .route("/", get(_read_all))
        .route("/", post(_create))
        .route("/:id", get(_read))
        .route("/:id", put(_update))
        .route("/:id", delete(_delete))
}

#[derive(Debug, Deserialize)]
struct Info {
    kind: AccountKind,
    name: String,
    description: Option<String>,
}

async fn _read_all(
    user_session: UserSession,
    State(account_ledger): State<AccountLedger>,
) -> impl IntoResponse {
    match account_ledger.read_all(user_session.id()).await {
        Ok(records) => todo!(),

        Err(error) => internal_error(error).into_response(),
    }
}

async fn _create(
    user_session: UserSession,
    State(account_ledger): State<AccountLedger>,
    IsHtmx(is_htmx): IsHtmx,
    Form(Info {
        kind,
        name,
        description,
    }): Form<Info>,
) -> impl IntoResponse {
    use crate::server::state::ledger::account::create::Error;

    match account_ledger
        .create(
            user_session.id(),
            kind,
            name.as_str(),
            description.as_deref(),
        )
        .await
    {
        Ok(record) => todo!(),

        Err(Error::Duplicate) => (StatusCode::CONFLICT, "account already exists").into_response(),
        Err(error) => internal_error(error).into_response(),
    }
}

async fn _read(
    user_session: UserSession,
    State(account_ledger): State<AccountLedger>,
    IsHtmx(is_htmx): IsHtmx,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    use crate::server::state::ledger::account::read::Error;

    match account_ledger.read(user_session.id(), id).await {
        Ok(record) => todo!(),

        Err(Error::NotFound) => (StatusCode::NOT_FOUND, "account not found").into_response(),
        Err(error) => internal_error(error).into_response(),
    }
}

async fn _update(
    user_session: UserSession,
    State(account_ledger): State<AccountLedger>,
    IsHtmx(is_htmx): IsHtmx,
    Path(id): Path<Uuid>,
    Form(Info {
        kind,
        name,
        description,
    }): Form<Info>,
) -> impl IntoResponse {
    use crate::server::state::ledger::account::update::Error;

    match account_ledger
        .update(
            user_session.id(),
            id,
            kind,
            name.as_str(),
            description.as_deref(),
        )
        .await
    {
        Ok(record) => todo!(),

        Err(Error::Duplicate) => (StatusCode::CONFLICT, "account already exists").into_response(),
        Err(Error::NotFound) => (StatusCode::NOT_FOUND, "account not found").into_response(),
        Err(error) => internal_error(error).into_response(),
    }
}

async fn _delete(
    user_session: UserSession,
    State(account_ledger): State<AccountLedger>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    use crate::server::state::ledger::account::delete::Error;

    match account_ledger.delete(user_session.id(), id).await {
        Ok(_) => todo!(),

        Err(Error::NotFound) => (StatusCode::NOT_FOUND, "account not found").into_response(),
        Err(error) => internal_error(error).into_response(),
    }
}
