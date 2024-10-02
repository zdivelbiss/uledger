use crate::{
    api::{
        internal_error,
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
    axum::Router::new().route("/", post(begin_verify).patch(finalize_verify))
}

#[axum::debug_handler]
async fn begin_verify(
    session: Session,
    user_state: State<UserState>,
    email_address: Json<EmailAddress>,
) -> impl IntoResponse {
    use crate::api::state::user::verify::Error;

    let Ok(Some(user_id)) = session.get("user_id").await else {
        return user_forbidden().into_response();
    };

    match user_state.begin_verify_email(user_id, &email_address).await {
        Ok(token) => {
            //     match send_verification(&email_address, token).await {
            //     Ok(_) => StatusCode::CREATED.into_response(),
            //     Err(err) => internal_error(err).into_response(),
            // }

            todo!()
        }

        Err(Error::UserNotExists) => StatusCode::NOT_FOUND.into_response(),
        Err(Error::EmailInUse) => StatusCode::CONFLICT.into_response(),

        Err(err) => internal_error(err).into_response(),
    }
}

#[axum::debug_handler]
async fn finalize_verify(
    user_state: State<UserState>,
    proof_token: Json<Uuid>,
) -> impl IntoResponse {
    StatusCode::IM_A_TEAPOT
}
