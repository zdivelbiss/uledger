use crate::{
    api::app_state::{users::UserState, verifications::VerificationState, AppState},
    util::EmailAddress,
};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json,
};
use serde::Deserialize;
use serde_big_array::BigArray;

pub fn routes() -> axum::Router<AppState> {
    axum::Router::new().route("/register", post(register))
}

#[derive(Debug, Deserialize)]
struct RegisterUser {
    email: EmailAddress,

    #[serde(with = "BigArray")]
    password_hash: [u8; 512],
}

async fn register(
    verifications_state: State<VerificationState>,
    register_user: Json<RegisterUser>,
) -> impl IntoResponse {
    use argon2::{
        password_hash::{rand_core::OsRng, SaltString},
        Argon2, PasswordHasher,
    };

    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let final_password_hash = argon2.hash_password(&register_user.password_hash, &salt);

    StatusCode::OK
}
