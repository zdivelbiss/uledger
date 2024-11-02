use crate::server::state::App;
use axum::{handler::Handler, Router};

mod v1;

pub fn router() -> Router<App> {
    Router::new().nest("/v1", v1::router())
}

pub fn crud_router<
    S: Clone + Send + Sync + 'static,
    TReadAll: 'static,
    TCreate: 'static,
    TRead: 'static,
    TUpdate: 'static,
    TDelete: 'static,
>(
    read_all: impl Handler<TReadAll, S>,
    create: impl Handler<TCreate, S>,
    read: impl Handler<TRead, S>,
    update: impl Handler<TUpdate, S>,
    delete: impl Handler<TDelete, S>,
) -> Router<S> {
    use axum::routing;

    Router::new()
        .route("/", routing::get(read_all))
        .route("/", routing::post(create))
        .route("/:id", routing::get(read))
        .route("/:id", routing::put(update))
        .route("/:id", routing::delete(delete))
}
