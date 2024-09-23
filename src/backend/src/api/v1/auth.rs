use crate::{
    api::app_state::{user_state::UserState, verify_state::VerifyState, AppState},
    util::EmailAddress,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json};
use base64::Engine;
use serde::Deserialize;

pub fn routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/register", post(register))
        .route("/verify", post(verify))
}

#[derive(Debug, Deserialize)]
struct RegisterUser {
    email_address: EmailAddress,
    password_digest: String,
}

#[axum::debug_handler]
async fn register(
    app_state: State<AppState>,
    register_user: Json<RegisterUser>,
) -> impl IntoResponse {
    use crate::api::app_state::user_state::Error;

    let user_state = app_state.user_state();
    let verify_state = app_state.verify_state();

    let mut password_digest = [0u8; 512];
    let base64_decode_result = base64::engine::general_purpose::STANDARD
        .decode_slice(register_user.password_digest.as_str(), &mut password_digest);
    if base64_decode_result.is_err() {
        return StatusCode::BAD_REQUEST.into_response();
    }

    let register_user_result = user_state
        .register_user(&register_user.email_address, &password_digest)
        .await;
    match register_user_result {
        Ok(id) => {
            let gen_email_verify_result = verify_state
                .gen_email_verify(&register_user.email_address)
                .await
                .map_err(|err| err.to_string());

            match gen_email_verify_result {
                Ok(token) => {
                    use crate::email::{send_verification, TEST_ENDPOINT};

                    if let Err(err) = send_verification(&TEST_ENDPOINT, token).await {
                        debug!("Error sending verification email: {err:?}");
                    }
                }

                Err(err) => panic!("ERR UNKNOWN: {err:?}"),
            }

            let id_serialized = serde_json::to_string(&id).unwrap();
            (StatusCode::OK, id_serialized).into_response()
        }

        Err(Error::UserExists) => StatusCode::CONFLICT.into_response(),
        Err(Error::Unknown) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

async fn verify(
    users_state: State<UserState>,
    verify_state: State<VerifyState>,
    register_user: Json<RegisterUser>,
) -> impl IntoResponse {
    todo!()
}
