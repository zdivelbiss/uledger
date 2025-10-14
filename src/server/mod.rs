use crate::{config::CONFIG, state::AppState};
use askama_web::WebTemplate;
use axum::{
    Router,
    http::{HeaderValue, StatusCode, header},
    response::IntoResponse,
};
use std::time::Duration;
use tokio::{net::TcpListener, time::timeout};
use tower_http::{
    compression::CompressionLayer, decompression::DecompressionLayer,
    set_header::SetResponseHeaderLayer, trace::TraceLayer,
};
use tower_sessions::{Expiry, SessionManagerLayer};
use tower_sessions_redis_store::{
    RedisStore,
    fred::{self, prelude::ClientLike},
};

mod api;
mod assets;
mod htmx;
mod site;

mod user_session;
pub use user_session::*;

#[derive(askama::Template)]
#[template(path = "singles/404.html")]
struct FallbackTemplate {}

pub async fn run() {
    let socket_bind = &CONFIG.network.bind;
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
    let app_state = AppState::create().await;

    // Connect to session database ...
    let url = CONFIG.session.url.as_str();
    debug!("Creating connection configuration for session database: {url:?}");
    let config = fred::types::config::Config::from_url(url).expect("invalid url");
    debug!("Building session database connection...");
    let client = fred::types::Builder::from_config(config).build().unwrap();
    debug!("Connecting to session database...");
    let _ = client.init().await;
    debug!("Session database connection established.");

    let session_expiry = Expiry::OnInactivity(CONFIG.session.lifetime.try_into().unwrap());
    debug!("Using session expiry: {session_expiry:?}");
    let session_store = RedisStore::new(client);
    info!("Session database conection ready.");

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(true)
        .with_http_only(true)
        .with_always_save(true)
        .with_expiry(session_expiry)
        .with_signed(CONFIG.session.key.clone());
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
        .merge(site::router())
        .nest("/api", api::router())
        .nest("/assets", assets::router())
        .fallback(|| async { (StatusCode::NOT_FOUND, WebTemplate(FallbackTemplate {})) })
        .layer(set_server_layer)
        .layer(compression_layer)
        .layer(decompression_layer)
        .layer(session_layer)
        .layer(TraceLayer::new_for_http())
        .with_state(app_state)
}

pub fn serialize_json_response<T: ?Sized + serde::Serialize>(value: &T) -> impl IntoResponse {
    match serde_json::to_string(value) {
        Ok(serialized) => (StatusCode::OK, serialized).into_response(),
        Err(error) => internal_error(error).into_response(),
    }
}

pub fn internal_error(error: impl std::error::Error) -> (StatusCode, &'static str) {
    error!("{error:?}");

    (StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
}
