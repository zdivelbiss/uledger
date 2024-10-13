#![allow(clippy::disallowed_types)]

use crate::server::state::AppState;
use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
    routing::get,
};
use tower_sessions::Session;

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

async fn redirect_unauthorized(session: Session, request: Request, next: Next) -> Response {
    let is_authenticated = session.get_value("user_id").await.ok().flatten().is_some();

    if is_authenticated {
        next.run(request).await
    } else {
        Redirect::temporary("/login").into_response()
    }
}
