mod accounts;

pub fn routes() -> axum::Router {
    axum::Router::new().nest("/accounts", accounts::routes())
}
