use crate::{
    server::{UserSession, api::crud_router, internal_error},
    state::{AppState, ledger::commodity::CommodityLedger},
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
    description: Option<String>,
    symbol: String,
    thousands_separator: String,
    decimal_separator: String,
    is_prefix: bool,
}

async fn _read_all(
    user_session: UserSession,
    State(ledger): State<CommodityLedger>,
) -> impl IntoResponse {
    match ledger.read_all(user_session.id()).await {
        Ok(records) => todo!(),

        Err(error) => internal_error(error).into_response(),
    }
}

async fn _create(
    user_session: UserSession,
    State(ledger): State<CommodityLedger>,
    Form(Info {
        name,
        description,
        symbol,
        thousands_separator,
        decimal_separator,
        is_prefix,
    }): Form<Info>,
) -> impl IntoResponse {
    use crate::state::ledger::commodity::create::Error;

    match ledger
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

        Err(Error::Duplicate) => (StatusCode::CONFLICT, "commodity already exists").into_response(),
        Err(Error::NameLength) => {
            (StatusCode::BAD_REQUEST, "commodity name too long").into_response()
        }
        Err(Error::DescriptionLength) => {
            (StatusCode::BAD_REQUEST, "commodity description too long").into_response()
        }
        Err(Error::SymbolLength) => {
            (StatusCode::BAD_REQUEST, "commodity symbol too long").into_response()
        }
        Err(Error::ThousandsSeparatorLength) => (
            StatusCode::BAD_REQUEST,
            "commodity thousands separator too long",
        )
            .into_response(),
        Err(Error::DecimalSeparatorLength) => (
            StatusCode::BAD_REQUEST,
            "commodity decimal separator too long",
        )
            .into_response(),
        Err(error) => internal_error(error).into_response(),
    }
}

async fn _read(
    user_session: UserSession,
    State(ledger): State<CommodityLedger>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    use crate::state::ledger::commodity::read::Error;

    match ledger.read(user_session.id(), id).await {
        Ok(record) => todo!(),

        Err(Error::NotFound) => (StatusCode::NOT_FOUND, "commodity not found").into_response(),
        Err(error) => internal_error(error).into_response(),
    }
}

async fn _update(
    user_session: UserSession,
    State(ledger): State<CommodityLedger>,
    Path(id): Path<Uuid>,
    Form(Info {
        name,
        description,
        symbol,
        thousands_separator,
        decimal_separator,
        is_prefix,
    }): Form<Info>,
) -> impl IntoResponse {
    use crate::state::ledger::commodity::update::Error;

    match ledger
        .update(
            user_session.id(),
            id,
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

        Err(Error::Duplicate) => (StatusCode::CONFLICT, "commodity already exists").into_response(),
        Err(Error::NameLength) => {
            (StatusCode::BAD_REQUEST, "commodity name too long").into_response()
        }
        Err(Error::DescriptionLength) => {
            (StatusCode::BAD_REQUEST, "commodity description too long").into_response()
        }
        Err(Error::SymbolLength) => {
            (StatusCode::BAD_REQUEST, "commodity symbol too long").into_response()
        }
        Err(Error::ThousandsSeparatorLength) => (
            StatusCode::BAD_REQUEST,
            "commodity thousands separator too long",
        )
            .into_response(),
        Err(Error::DecimalSeparatorLength) => (
            StatusCode::BAD_REQUEST,
            "commodity decimal separator too long",
        )
            .into_response(),
        Err(error) => internal_error(error).into_response(),
    }
}

async fn _delete(
    user_session: UserSession,
    State(ledger): State<CommodityLedger>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    use crate::state::ledger::commodity::delete::Error;

    match ledger.delete(user_session.id(), id).await {
        Ok(_) => todo!(),

        Err(Error::NotFound) => (StatusCode::NOT_FOUND, "commodity not found").into_response(),
        Err(error) => internal_error(error).into_response(),
    }
}
