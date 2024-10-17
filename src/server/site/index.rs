use crate::server::user_session::UserSession;

#[derive(askama::Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    display_name: String,
    // accounts: Box<[()]>,
}

#[axum::debug_handler]
pub async fn serve(user_session: UserSession) -> IndexTemplate {
    IndexTemplate {
        display_name: user_session.get_display_name().await,
    }
}
