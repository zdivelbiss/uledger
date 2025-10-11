use crate::{
    server::{UserSession, api::crud_router, internal_error, serialize_json_response},
    state::{AppState, ledger::transaction::TransactionLedger}, util::CurrencyCode,
};
use axum::{
    Router,
    extract::{Form, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::NaiveDate;
use uuid::Uuid;

pub fn router() -> Router<AppState> {
    crud_router(_read_all, _create, _read, _update, _delete)
}

#[derive(Debug, Deserialize)]
struct Info {
    occurred_on: NaiveDate,
    account: Uuid,
    payee: Uuid,
    currency: CurrencyCode,
    amount: f64,
    description: Option<String>,
}

async fn _read_all(
    user_session: UserSession,
    State(ledger): State<TransactionLedger>,
) -> impl IntoResponse {
    match ledger.read_all(user_session.id()).await {
        Ok(records) => serialize_json_response(&records).into_response(),

        Err(error) => internal_error(error).into_response(),
    }
}

async fn _create(
    user_session: UserSession,
    State(ledger): State<TransactionLedger>,
    Form(Info {
        occurred_on,
        account,
        payee,
        currency,
        amount,
        description,
    }): Form<Info>,
) -> impl IntoResponse {
    use crate::state::ledger::transaction::create::Error;

    match ledger
        .create(
            user_session.id(),
            occurred_on,
            account,
            payee,
            currency,
            amount,
            description.as_deref(),
        )
        .await
    {
        Ok(record) => serialize_json_response(&record).into_response(),

        Err(Error::CurrencyCodeLength) => {
            (StatusCode::BAD_REQUEST, "Currency code is too long.").into_response()
        }

        Err(Error::CurrencyCodeSupport) => {
            (StatusCode::BAD_REQUEST, "Currency code is not supported.").into_response()
        }

        Err(Error::DescriptionLength) => {
            (StatusCode::BAD_REQUEST, "Description is too long.").into_response()
        }

        Err(error) => internal_error(error).into_response(),
    }
}

async fn _read(
    user_session: UserSession,
    State(ledger): State<TransactionLedger>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    use crate::state::ledger::transaction::read::Error;

    match ledger.read(user_session.id(), id).await {
        Ok(record) => serialize_json_response(&record).into_response(),

        Err(Error::NotFound) => {
            (StatusCode::NOT_FOUND, "Transaction was not found.").into_response()
        }
        Err(error) => internal_error(error).into_response(),
    }
}

async fn _update(
    user_session: UserSession,
    State(ledger): State<TransactionLedger>,
    Path(id): Path<Uuid>,
    Form(Info {
        occurred_on,
        account,
        payee,
        currency,
        amount,
        description,
    }): Form<Info>,
) -> impl IntoResponse {
    use crate::state::ledger::transaction::update::Error;

    match ledger
        .update(
            user_session.id(),
            id,
            occurred_on,
            account,
            payee,
            currency,
            amount,
            description.as_deref(),
        )
        .await
    {
        Ok(record) => serialize_json_response(&record).into_response(),

        Err(Error::NotFound) => {
            (StatusCode::NOT_FOUND, "Transaction was not found.").into_response()
        }
        Err(Error::Duplicate) => {
            (StatusCode::CONFLICT, "Transaction already exists.").into_response()
        }
        Err(Error::NameLength) => {
            (StatusCode::BAD_REQUEST, "Transaction name is too long.").into_response()
        }
        Err(Error::DescriptionLength) => (
            StatusCode::BAD_REQUEST,
            "Transaction description is too long.",
        )
            .into_response(),
        Err(error) => internal_error(error).into_response(),
    }
}

async fn _delete(
    user_session: UserSession,
    State(ledger): State<TransactionLedger>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    use crate::state::ledger::transaction::delete::Error;

    match ledger.delete(user_session.id(), id).await {
        Ok(_) => (StatusCode::OK, "Transaction has been deleted.").into_response(),

        Err(Error::NotFound) => {
            (StatusCode::NOT_FOUND, "Transaction was not found.").into_response()
        }
        Err(error) => internal_error(error).into_response(),
    }
}
