use crate::{server::UserSession, state::AppState};
use askama_web::WebTemplate;
use axum::{
    Router,
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Redirect},
    routing::get,
};

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
#[template(path = "pages/payees.html")]
struct PayeesTemplate {}

pub fn router() -> Router<AppState> {
    Router::new()
        // Authenticated
        .route("/", get(|| async { Redirect::temporary("/accounts") }))
        .route(
            "/accounts",
            get(|| async { WebTemplate(AccountsTemplate {}) }),
        )
        .route("/payees", get(|| async { WebTemplate(PayeesTemplate {}) }))
        .layer(axum::middleware::from_fn(check_user_authenticated))
        // Unauthenticated
        .route(
            "/register",
            get(|| async { WebTemplate(RegisterTemplate {}) }),
        )
        .route("/login", get(|| async { WebTemplate(LoginTemplate {}) }))
}

async fn check_user_authenticated(
    user_session: Option<UserSession>,
    request: Request,
    next: Next,
) -> impl IntoResponse {
    if user_session.is_some() {
        next.run(request).await
    } else {
        Redirect::temporary("/login").into_response()
    }
}
