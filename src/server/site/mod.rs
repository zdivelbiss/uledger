use crate::server::state::AppState;
use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
    routing::get,
};

use super::user_session::UserSession;

mod index;

#[derive(Template)]
#[template(path = "register.html")]
pub struct RegisterTemplate {}

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {}

pub fn router() -> axum::Router<AppState> {
    axum::Router::new()
        // Authenticated
        .route("/", get(index::serve))
        .layer(axum::middleware::from_fn(redirect_unauthorized))
        // Unauthenticated
        .route("/register", get(|| async { RegisterTemplate {} }))
        .route("/login", get(|| async { LoginTemplate {} }))
}

async fn redirect_unauthorized(
    user_session: Option<UserSession>,
    request: Request,
    next: Next,
) -> Response {
    if user_session.is_some() {
        next.run(request).await
    } else {
        Redirect::temporary("/login").into_response()
    }
}
