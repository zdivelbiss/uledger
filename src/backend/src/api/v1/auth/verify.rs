use crate::{
    api::app_state::{user_state::UserState, AppState},
    util::EmailAddress,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;

pub fn routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/create", post(create_verify))
        //.route("/transmit", post(transmit_verify))
        .route("/finalize", post(finalize_verify))
}

async fn create_verify(
    user_id: Path<Uuid>,
    user_state: State<UserState>,
    email_address: Json<EmailAddress>,
) -> impl IntoResponse {
    use crate::api::app_state::user_state::create_verify_email::Error;

    user_state
        .create_verify_email(*user_id, &email_address)
        .await
        .map_or_else(
            |err| match err {
                Error::UserNotExists => StatusCode::NOT_FOUND,
                Error::EmailInUse => StatusCode::CONFLICT,

                _ => {
                    error!("{err:?}");

                    StatusCode::INTERNAL_SERVER_ERROR
                }
            },
            |_| StatusCode::CREATED,
        )
}

#[derive(Debug, Deserialize)]
struct VerifyEmailAddress {
    email_address: EmailAddress,
    proof_token: Uuid,
}

async fn finalize_verify(
    user_state: State<UserState>,
    body: Json<VerifyEmailAddress>,
) -> impl IntoResponse {
    StatusCode::IM_A_TEAPOT
}
