use crate::cfg;
use axum::{
    http::{header, HeaderValue, StatusCode},
    Router,
};
use std::time::Duration;
use tokio::{net::TcpListener, time::timeout};
use tower_http::{
    compression::CompressionLayer, decompression::DecompressionLayer,
    set_header::SetResponseHeaderLayer,
};
use tower_sessions::{Expiry, Session, SessionManagerLayer};
use tower_sessions_redis_store::{
    fred::{self, prelude::ClientLike},
    RedisStore,
};
use uuid::Uuid;

mod api;
mod htmx;
mod responses;
mod site;
mod state;

#[derive(askama::Template)]
#[template(path = "404.html")]
struct FallbackTemplate {}

pub async fn run() {
    let socket_bind = &crate::cfg().network.bind;
    info!("Binding listener: http://{socket_bind}/");

    let app = build_router().await;
    let listener = timeout(Duration::from_secs(5), TcpListener::bind(socket_bind))
        .await
        .expect("timed out attempting to bind socket")
        .expect("error binding socket");

    info!("Begin listening for requests.");
    axum::serve(listener, app)
        .await
        .expect("error serving connections");
}

async fn build_router() -> Router {
    let state = state::AppState::create().await;

    let url = cfg().session.url.as_str();

    debug!("Creating connection configuration for session storage: {url:?}");
    let config = fred::types::RedisConfig::from_url(url).expect("invalid url");
    debug!("Building session storage connection...");
    let client = fred::types::Builder::from_config(config)
        .build()
        .expect("could not crate");
    debug!("Connecting to session storage...");
    let _ = client.init().await;
    debug!("Session storage connection established.");

    let session_expiry = Expiry::OnInactivity(cfg().session.lifetime.try_into().unwrap());
    debug!("Using session expiry: {session_expiry:?}");
    let session_store = RedisStore::new(client);

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(true)
        .with_http_only(true)
        .with_always_save(true)
        .with_expiry(session_expiry)
        .with_signed(cfg().session.key.clone());
    let decompression_layer = DecompressionLayer::new()
        .zstd(true)
        .br(true)
        .gzip(true)
        .no_deflate();
    let compression_layer = CompressionLayer::new()
        .zstd(true)
        .br(true)
        .gzip(true)
        .no_deflate();
    let set_server_layer = SetResponseHeaderLayer::if_not_present(
        header::SERVER,
        HeaderValue::from_static(crate::user_agent()),
    );

    Router::new()
        .nest("/", site::router())
        .nest("/api", api::router())
        .fallback(|| async { FallbackTemplate {} })
        .layer(set_server_layer)
        .layer(compression_layer)
        .layer(decompression_layer)
        .layer(session_layer)
        .with_state(state)
}

pub async fn is_authenticated(session: &Session) -> bool {
    session
        .get::<Uuid>("user_id")
        .await
        .ok()
        .flatten()
        .is_some()
}

pub async fn authentication_layer(
    session: Session,
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    use axum::response::IntoResponse;

    if is_authenticated(&session).await {
        next.run(request).await
    } else {
        (
            axum::http::StatusCode::UNAUTHORIZED,
            "You must authenticate.",
        )
            .into_response()
    }
}

pub fn internal_error(error: impl std::fmt::Debug) -> StatusCode {
    error!("{error:?}");

    StatusCode::INTERNAL_SERVER_ERROR
}
