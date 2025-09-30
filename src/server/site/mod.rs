use crate::server::{state::App, UserSession};
use askama_web::WebTemplate;
use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Redirect},
    routing::get,
};

// mod index;
// mod pages;

#[derive(askama::Template)]
#[template(path = "singles/register.html")]
pub struct RegisterTemplate {}

#[derive(askama::Template)]
#[template(path = "singles/login.html")]
pub struct LoginTemplate {}

#[derive(askama::Template)]
#[template(path = "pages/accounts.html")]
struct AccountsTemplate {}

#[derive(askama::Template)]
#[template(path = "pages/commodities.html")]
struct CommoditiesTemplate {}

pub fn router() -> axum::Router<App> {
    axum::Router::new()
        // Authenticated
        .route("/", get(|| async { Redirect::temporary("/accounts") }))
        .route(
            "/accounts",
            get(|| async { WebTemplate(AccountsTemplate {}) }),
        )
        .route(
            "/commodities",
            get(|| async { WebTemplate(CommoditiesTemplate {}) }),
        )
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
        .route(
            "/register",
            get(|| async { WebTemplate(RegisterTemplate {}) }),
        )
        .route("/login", get(|| async { WebTemplate(LoginTemplate {}) }))
}
