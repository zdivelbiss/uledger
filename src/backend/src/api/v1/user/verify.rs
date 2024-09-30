use crate::{
    api::{
        state::{user::UserState, AppState},
        user_forbidden,
    },
    util::EmailAddress,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json};
use serde::Deserialize;
use tower_sessions::Session;
use uuid::Uuid;

pub fn routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/create", post(create_verify))
        //.route("/transmit", post(transmit_verify))
        .route("/finalize", post(finalize_verify))
}

#[axum::debug_handler]
async fn create_verify(
    session: Session,
    user_state: State<UserState>,
    email_address: Json<EmailAddress>,
) -> impl IntoResponse {
    use crate::api::state::user::verify::Error;

    let Ok(Some(user_id)) = session.get("user_id").await else {
        return user_forbidden().into_response();
    };

    match user_state
        .create_verify_email(user_id, &email_address)
        .await
    {
        Ok(_) => StatusCode::CREATED.into_response(),

        Err(Error::UserNotExists) => StatusCode::NOT_FOUND.into_response(),
        Err(Error::EmailInUse) => StatusCode::CONFLICT.into_response(),

        Err(err) => {
            error!("{err:?}");

            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

#[derive(Debug, Deserialize)]
struct VerifyEmailAddress {
    email_address: EmailAddress,
    proof_token: Uuid,
}

#[axum::debug_handler]
async fn finalize_verify(
    user_state: State<UserState>,
    body: Json<VerifyEmailAddress>,
) -> impl IntoResponse {
    StatusCode::IM_A_TEAPOT
}
