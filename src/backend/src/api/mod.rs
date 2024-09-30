use crate::{api::state::AppState, cfg};
use axum::{
    http::{header, HeaderValue, StatusCode},
    response::IntoResponse,
    Router,
};
use base64::Engine;
use std::time::Duration;
use tokio::{net::TcpListener, time::timeout};
use tower_http::{
    compression::CompressionLayer, decompression::DecompressionLayer,
    set_header::SetResponseHeaderLayer,
};
use tower_sessions::{cookie, Expiry, Session, SessionManagerLayer};
use tower_sessions_redis_store::{
    fred::{self, prelude::ClientLike},
    RedisStore,
};
use uuid::Uuid;

mod services;
mod state;
mod v1;

pub async fn accept_connections() {
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
        .with_always_save(true)
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

pub fn internal_error(err: impl std::fmt::Debug) -> impl IntoResponse {
    error!("{err:?}");

    StatusCode::INTERNAL_SERVER_ERROR.into_response()
}

pub fn user_forbidden() -> impl IntoResponse {
    (StatusCode::FORBIDDEN, "You must authenticate.")
}

pub async fn init_session(
    user_id: Uuid,
    user_agent: Option<&str>,
    session: &Session,
) -> Result<(), tower_sessions::session::Error> {
    session.insert("user_id", user_id).await?;
    session.insert("user_agent", user_agent).await?;

    Ok(())
}

pub async fn is_matching_session(user_id: Uuid, session: &Session) -> bool {
    session
        .get::<Uuid>("user_id")
        .await
        .unwrap_or(None)
        .map(|session_user_id| session_user_id == user_id)
        .unwrap_or(false)
}
