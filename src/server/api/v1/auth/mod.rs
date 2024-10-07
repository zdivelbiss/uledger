use crate::{
    server::state::AppState,
    util::{EmailAddress, PasswordDigest},
};
use axum::routing::post;
use serde::Deserialize;

mod login;
mod logout;
mod register;

pub fn router() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/register", post(register::register))
        .route("/login", post(login::login))
        .route("/logout", post(logout::logout))
}

#[derive(Debug, Deserialize)]
pub struct AuthInfo {
    email_address: EmailAddress,
    password_digest: PasswordDigest,
}
