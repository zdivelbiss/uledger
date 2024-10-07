use crate::cfg;
use axum::{
    http::{header, HeaderValue},
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
mod responses;
mod state;
mod web;

pub async fn run() {
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

    let app = Router::new()
        .layer(decompression_layer)
        .nest("/api", api::router())
        .nest("/", web::router())
        .layer(set_server_layer)
        .layer(compression_layer)
        .layer(session_layer)
        .with_state(state);

    let socket_bind = &crate::cfg().network.bind;
    info!("Binding listener: http://{socket_bind}/");

    let listener = timeout(Duration::from_secs(5), TcpListener::bind(socket_bind))
        .await
        .expect("timed out attempting to bind socket")
        .expect("error binding socket");

    info!("Begin listening for requests.");
    axum::serve(listener, app)
        .await
        .expect("error serving connections");
}

pub async fn get_user_id(session: &Session) -> Option<Uuid> {
    session
        .get("user_id")
        .await
        .expect("error with session storage")
}
