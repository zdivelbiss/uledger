use crate::server::{AppState, UserSession};
use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Redirect},
    routing::get,
};

// mod index;
// mod pages;

#[derive(askama::Template)]
#[template(path = "auth/register.html")]
pub struct RegisterTemplate {}

#[derive(askama::Template)]
#[template(path = "auth/login.html")]
pub struct LoginTemplate {}

#[derive(askama::Template)]
#[template(path = "pages/accounts.html")]
struct AccountsTemplate {}

#[derive(askama::Template)]
#[template(path = "pages/commodities.html")]
struct CommoditiesTemplate {}

pub fn router() -> axum::Router<AppState> {
    axum::Router::new()
        // Authenticated
        .route("/", get(|| async { Redirect::temporary("/accounts") }))
        .route("/accounts", get(|| async { AccountsTemplate {} }))
        .route("/commodities", get(|| async { CommoditiesTemplate {} }))
        .layer(axum::middleware::from_fn(
            |user_session: Option<UserSession>, request: Request, next: Next| async move {
                if user_session.is_none() {
                    Redirect::temporary("/login").into_response()
                } else {
                    next.run(request).await
                }
            },
        ))
        // Unauthenticated
        .route("/register", get(|| async { RegisterTemplate {} }))
        .route("/login", get(|| async { LoginTemplate {} }))
}
