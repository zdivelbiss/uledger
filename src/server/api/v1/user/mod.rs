use crate::server::state::AppState;
use axum::routing::post;

mod login;
mod logout;
mod register;
mod verify;

pub fn router() -> axum::Router<AppState> {
    // TODO abstract the behaviour of these functions into a UserState object

    axum::Router::new()
        .nest("/verify", verify::router())
        .route("/register", post(register::handler))
        .route("/login", post(login::handler))
        .route("/logout", post(logout::handler))
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
