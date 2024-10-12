use crate::{util::EmailAddress, server::state::AppState};
use axum::routing::post;

mod login;
mod logout;
mod register;
mod verify;

pub fn router() -> axum::Router<AppState> {
    axum::Router::new()
        .nest("/verify", verify::router())
        .layer(axum::middleware::from_fn(crate::server::authentication_layer))
        .route("/register", post(register::handler))
        .route("/login", post(login::handler))
        .route("/logout", post(logout::handler))
}

#[derive(Debug, Deserialize)]
pub struct AuthInfo {
    email_address: EmailAddress,
    password: String,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, FromPrimitive, Serialize, Deserialize,
)]
#[serde(rename_all = "UPPERCASE")]
#[repr(i16)]
pub enum Role {
    Admin = 0,

    #[default]
    Regular = 100,
}
