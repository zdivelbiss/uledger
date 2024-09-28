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
use base64::Engine;
use serde::Deserialize;
use uuid::Uuid;

mod verify;

pub fn routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/register", post(register))
        .route("/:user_id/login", post(register))
        .nest("/:user_id/verify", verify::routes())
}

#[derive(Debug, Deserialize)]
struct RegisterUser {
    email_address: EmailAddress,
    password_digest: String,
}

#[axum::debug_handler]
async fn register(user_state: State<UserState>, body: Json<RegisterUser>) -> impl IntoResponse {
    use crate::api::app_state::user_state::register_user::Error;

    let mut password_digest = [0u8; 512];
    if base64::engine::general_purpose::STANDARD
        .decode_slice(body.password_digest.as_str(), &mut password_digest)
        .is_err()
    {
        return StatusCode::BAD_REQUEST.into_response();
    }

    user_state
        .register_user(&body.email_address, &password_digest)
        .await
        .map_or_else(
            |err| match err {
                Error::UserExists => StatusCode::CONFLICT.into_response(),

                _ => {
                    error!("{err:?}");

                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            },
            |id| {
                let id_serialized = serde_json::to_string(&id).unwrap();
                (StatusCode::OK, id_serialized).into_response()
            },
        )
}
