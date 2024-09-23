use crate::{
    api::app_state::{user_state::UserState, verify_state::VerifyState, AppState},
    util::EmailAddress,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json};
use base64::Engine;
use serde::Deserialize;

pub fn routes() -> axum::Router<AppState> {
    axum::Router::new().route("/register", post(register))
}

#[derive(Debug, Deserialize)]
struct RegisterUser {
    email: EmailAddress,
    password_digest: String,
}

async fn register(
    users_state: State<UserState>,
    register_user: Json<RegisterUser>,
) -> impl IntoResponse {
    use crate::api::app_state::user_state::Error;

    // TODO error checking
    let mut password_digest = [0u8; 512];
    base64::engine::general_purpose::STANDARD
        .decode_slice(register_user.password_digest.as_str(), &mut password_digest)
        .unwrap();

    let result = users_state
        .register_user(&register_user.email, &password_digest)
        .await;

    match result {
        Ok(id) => {
            let id_serialized = serde_json::to_string(&id).unwrap();
            (StatusCode::OK, id_serialized).into_response()
        }

        Err(Error::UserExists) => StatusCode::CONFLICT.into_response(),

        Err(Error::Unknown) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
