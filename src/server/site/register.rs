use axum::response::IntoResponse;

#[derive(Template)]
#[template(path = "register.html")]
pub struct RegisterTemplate {}

#[axum::debug_handler]
pub async fn serve(session: tower_sessions::Session) -> impl IntoResponse {
    if crate::server::is_authenticated(&session).await {
        crate::server::redirect_root().into_response()
    } else {
        (RegisterTemplate {}).into_response()
    }
}
