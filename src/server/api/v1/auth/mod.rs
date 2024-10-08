use crate::{server::state::AppState, util::EmailAddress};
use axum::routing::post;

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
