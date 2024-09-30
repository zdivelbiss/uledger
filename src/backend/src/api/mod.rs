use crate::{api::state::AppState, cfg};
use axum::{
    http::{header, HeaderValue},
    Router,
};
use base64::Engine;
use std::time::Duration;
use tokio::{net::TcpListener, time::timeout};
use tower_http::{
    compression::CompressionLayer, decompression::DecompressionLayer,
    set_header::SetResponseHeaderLayer,
};
use tower_sessions::{cookie, Expiry, SessionManagerLayer};
use tower_sessions_redis_store::{
    fred::{self},
    RedisStore,
};

mod services;
mod state;
mod v1;

pub async fn accept_connections() {
    let state = AppState::create().await;

    let url = cfg().session.url.as_str();

    let config = fred::types::RedisConfig::from_url(url).expect("invalid url");
    let client = fred::types::Builder::from_config(config)
        .build()
        .expect("could not connect");

    let key_bytes = base64::engine::general_purpose::STANDARD
        .decode(cfg().session.apikey.as_str())
        .expect("failed to decode BASE64 cookies API key");

    let session_key = cookie::Key::try_from(key_bytes.as_slice()).expect("session key is invalid");
    let session_lifetime = cfg().session.lifetime;
    let session_expiry = Expiry::OnInactivity(session_lifetime.try_into().unwrap());
    let session_store = RedisStore::new(client);

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(true)
        .with_http_only(true)
        .with_expiry(session_expiry)
        .with_private(session_key);
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
        .nest("/api/v1", v1::routes())
        .layer(set_server_layer)
        .layer(compression_layer)
        .layer(session_layer)
        .with_state(state);

    // TODO error handling
    let listener = timeout(
        Duration::from_secs(5),
        TcpListener::bind(crate::cfg().network.bind),
    )
    .await
    .unwrap()
    .unwrap();

    info!("Begin listening for requests.");
    axum::serve(listener, app)
        .await
        .expect("error serving connections");
}
