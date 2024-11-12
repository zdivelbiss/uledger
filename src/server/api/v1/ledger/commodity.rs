use crate::server::{
    api::crud_router,
    htmx::IsHtmx,
    internal_error,
    state::{
        ledger::{account::AccountLedger, commodity::CommodityLedger},
        App,
    },
    UserSession,
};
use axum::{
    extract::{Form, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Router,
};
use uuid::Uuid;

pub fn router() -> Router<App> {
    crud_router(_read_all, _create, _read, _update, _delete)
}

#[derive(Debug, Deserialize)]
struct Info {
    name: String,
    description: Option<String>,
    symbol: String,
    thousands_separator: String,
    decimal_separator: String,
    is_prefix: bool,
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
    State(commodity_ledger): State<CommodityLedger>,
    IsHtmx(is_htmx): IsHtmx,
    Form(Info {
        name,
        description,
        symbol,
        thousands_separator,
        decimal_separator,
        is_prefix,
    }): Form<Info>,
) -> impl IntoResponse {
    use crate::server::state::ledger::commodity::create::Error;

    match commodity_ledger
        .create(
            user_session.id(),
            name.as_str(),
            description.as_deref(),
            symbol.as_str(),
            thousands_separator.as_str(),
            decimal_separator.as_str(),
            is_prefix,
        )
        .await
    {
        Ok(record) => todo!(),

        Err(Error::Duplicate) => (StatusCode::CONFLICT, "account already exists").into_response(),
        Err(Error::NameLength) => (StatusCode::BAD_REQUEST, "name too long").into_response(),
        Err(Error::DescriptionLength) => {
            (StatusCode::BAD_REQUEST, "description too long").into_response()
        }
        Err(Error::SymbolLength) => (StatusCode::BAD_REQUEST, "symbol too long").into_response(),
        Err(Error::ThousandsSeparatorLength) => {
            (StatusCode::BAD_REQUEST, "thousands separator too long").into_response()
        }
        Err(Error::DecimalSeparatorLength) => {
            (StatusCode::BAD_REQUEST, "decimal separator too long").into_response()
        }
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
