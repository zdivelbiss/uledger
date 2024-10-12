
use crate::server::state::AppState;
use axum::routing::get;
use tower_sessions::Session;

mod index;
mod login;

pub fn router() -> axum::Router<AppState> {
    axum::Router::new()
        // Authenticated
        .route("/", get(index::serve))
        .layer(axum::middleware::from_fn(redirect_unauthorized))
        // Unauthenticated
        .route("/login", get(login::serve))
}

async fn redirect_unauthorized(
    session: Session,
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    use axum::response::IntoResponse;

    if crate::server::is_authenticated(&session).await {
        next.run(request).await
    } else {
        axum::response::Redirect::temporary("/login").into_response()
    }
}
