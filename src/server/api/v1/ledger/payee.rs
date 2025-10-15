use crate::{
    server::{
        UserSession, api::crud_router, htmx::hx_trigger, internal_error, serialize_json_response,
    },
    state::{
        AppState,
        ledger::payee::{PayeeLedger, PayeeRecord},
    },
};
use axum::{
    Router,
    extract::{Form, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

const HTMX_PAYEES_CHANGED: &str = "eventPayeesChanged";

pub fn router() -> Router<AppState> {
    crud_router(_read_all, _create, _read, _update, _delete)
}

#[derive(Debug, Deserialize)]
struct Info {
    name: String,
    description: Option<String>,
}

async fn _read_all(
    user_session: UserSession,
    State(ledger): State<PayeeLedger>,
) -> impl IntoResponse {
    match ledger.read_all(user_session.id()).await {
        Ok(records) => serialize_json_response(&records).into_response(),
        Err(error) => internal_error(error).into_response(),
    }
}

fn record_response(record: PayeeRecord) -> impl IntoResponse {
    match serde_json::to_string(&record) {
        Ok(serialized) => (
            StatusCode::OK,
            [hx_trigger(HTMX_PAYEES_CHANGED)],
            serialized,
        )
            .into_response(),

        Err(error) => internal_error(error).into_response(),
    }
}

async fn _create(
    user_session: UserSession,
    State(ledger): State<PayeeLedger>,

    Form(Info { name, description }): Form<Info>,
) -> impl IntoResponse {
    use crate::state::ledger::payee::create::Error;

    match ledger
        .create(user_session.id(), name.as_str(), description.as_deref())
        .await
    {
        Ok(record) => record_response(record).into_response(),

        Err(Error::Duplicate) => (StatusCode::CONFLICT, "Payee already exists.").into_response(),
        Err(Error::NameLength) => (StatusCode::BAD_REQUEST, "Payee name too long.").into_response(),
        Err(Error::DescriptionLength) => {
            (StatusCode::BAD_REQUEST, "Payee description too long.").into_response()
        }
        Err(error) => internal_error(error).into_response(),
    }
}

async fn _read(
    user_session: UserSession,
    State(ledger): State<PayeeLedger>,

    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    use crate::state::ledger::payee::read::Error;

    match ledger.read(user_session.id(), id).await {
        Ok(record) => record_response(record).into_response(),

        Err(Error::NotFound) => (StatusCode::NOT_FOUND, "Payee not found.").into_response(),
        Err(error) => internal_error(error).into_response(),
    }
}

async fn _update(
    user_session: UserSession,
    State(ledger): State<PayeeLedger>,

    Path(id): Path<Uuid>,
    Form(Info { name, description }): Form<Info>,
) -> impl IntoResponse {
    use crate::state::ledger::payee::update::Error;

    match ledger
        .update(user_session.id(), id, name.as_str(), description.as_deref())
        .await
    {
        Ok(record) => record_response(record).into_response(),

        Err(Error::NotFound) => (StatusCode::NOT_FOUND, "Payee not found.").into_response(),
        Err(Error::Duplicate) => (StatusCode::CONFLICT, "Payee already exists.").into_response(),
        Err(Error::NameLength | Error::DescriptionLength) => {
            (StatusCode::BAD_REQUEST, "Payee not found.").into_response()
        }

        Err(error) => internal_error(error).into_response(),
    }
}

async fn _delete(
    user_session: UserSession,
    State(ledger): State<PayeeLedger>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    use crate::state::ledger::payee::delete::Error;

    match ledger.delete(user_session.id(), id).await {
        Ok(_) => (
            StatusCode::OK,
            [hx_trigger(HTMX_PAYEES_CHANGED)],
            "Payee has been deleted.",
        )
            .into_response(),

        Err(Error::NotFound) => (StatusCode::NOT_FOUND, "Payee not found.").into_response(),
        Err(error) => internal_error(error).into_response(),
    }
}
