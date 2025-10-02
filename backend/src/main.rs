#![allow(clippy::unused_unit, clippy::too_many_arguments)]
#![deny(
    clippy::disallowed_types,
    clippy::disallowed_macros,
    clippy::disallowed_methods,
    clippy::disallowed_names
)]

mod api;
mod config;
// mod postmark;
mod state;
mod user_session;
mod util;
mod email;

use axum::{
    Router,
    http::{HeaderValue, StatusCode, header},
};
use std::time::Duration;
use tokio::net::TcpListener;
use tower_http::{
    compression::CompressionLayer, decompression::DecompressionLayer,
    set_header::SetResponseHeaderLayer,
};
use tower_sessions::SessionManagerLayer;
use tower_sessions_redis_store::{RedisStore, fred::prelude::ClientLike};

#[macro_use]
extern crate tracing;

#[macro_use]
extern crate sqlx;

#[macro_use]
extern crate serde;

pub type Datastore = sqlx::Pool<sqlx::Postgres>;

const fn user_agent() -> &'static str {
    concat!("uledger-srv/", env!("CARGO_PKG_VERSION"))
}

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    {
        dotenvy::dotenv().expect("no `.env` file");

        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();
    }
    #[cfg(not(debug_assertions))]
    {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .init();
    }

    let socket_bind = &crate::config::get().network.bind;
    info!("Binding listener: http://{socket_bind}/");

    let app = build_router().await;
    let listener = tokio::time::timeout(Duration::from_secs(5), TcpListener::bind(socket_bind))
        .await
        .expect("timed out attempting to bind socket")
        .expect("error binding socket");

    info!("Begin listening for requests.");
    axum::serve(listener, app)
        .await
        .expect("error serving connections");

    info!("Reached safe shutdown point.");
}

async fn build_router() -> Router {
    let state = crate::state::App::create().await;

    // Connect to session database ...
    let url = config::get().session.url.as_str();
    debug!("Creating connection configuration for session database: {url:?}");
    let cache_config = tower_sessions_redis_store::fred::types::config::Config::from_url(url)
        .expect("invalid url");
    debug!("Building session database connection...");
    let cache_client = tower_sessions_redis_store::fred::types::Builder::from_config(cache_config)
        .build()
        .unwrap();
    debug!("Connecting to session database...");
    let _ = cache_client.init().await;
    debug!("Session database connection established.");

    let session_expiry =
        tower_sessions::Expiry::OnInactivity(config::get().session.lifetime.try_into().unwrap());
    debug!("Using session expiry: {session_expiry:?}");
    let session_store = RedisStore::new(cache_client);
    info!("Session database conection ready.");

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(true)
        .with_http_only(true)
        .with_always_save(true)
        .with_expiry(session_expiry)
        .with_signed(config::get().session.key.clone());
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
        .nest("/api", api::router())
        .fallback(|| async { StatusCode::NOT_FOUND })
        .layer(set_server_layer)
        .layer(compression_layer)
        .layer(decompression_layer)
        .layer(session_layer)
        .with_state(state)
}

pub fn internal_error(error: impl std::error::Error) -> (StatusCode, &'static str) {
    error!("{error:?}");

    (StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
}
