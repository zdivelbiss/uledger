use crate::{api::state::AppState, cfg};
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

mod state;
mod v1;

pub async fn run_server() {
    let state = AppState::create().await;

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
        .nest("/api/v1", v1::router())
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

pub fn internal_error(error: impl std::fmt::Debug) -> StatusCode {
    error!("{error:?}");

    StatusCode::INTERNAL_SERVER_ERROR
}

#[derive(Debug)]
struct NoUserIdError;

impl std::fmt::Display for NoUserIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "user is not authenticated")
    }
}

impl std::error::Error for NoUserIdError {}

pub async fn get_user_id(session: &Session) -> Result<Uuid, NoUserIdError> {
    match session.get("user_id").await {
        Ok(Some(user_id)) => Ok(user_id),
        _ => Err(NoUserIdError),
    }
}
