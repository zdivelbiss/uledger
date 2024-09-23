use crate::{api::state::State, util::EmailAddress};
use axum::{
    extract::Query,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json,
};
use serde::Deserialize;
use serde_big_array::BigArray;

pub fn routes() -> axum::Router<State> {
    axum::Router::new().route("/register", post(register))
}

#[derive(Debug, Deserialize)]
struct RegisterUser {
    email: EmailAddress,

    #[serde(with = "BigArray")]
    password_hash: [u8; 512],
}

async fn register(register_user: Json<RegisterUser>) -> impl IntoResponse {
    use sha3::{Digest, Sha3_512};
    let mut hasher = Sha3_512::new();
    hasher.update("asdasdas");
    let hash = hasher.finalize();

    StatusCode::OK
}
