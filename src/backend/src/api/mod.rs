use std::time::Duration;

use axum::{
    http::{header, HeaderValue},
    Router,
};
use tokio::{net::TcpListener, time::timeout};
use tower_http::{
    compression::CompressionLayer, decompression::DecompressionLayer,
    set_header::SetResponseHeaderLayer,
};

mod app_state;
mod v1;

pub async fn accept_connections() {
    let state = app_state::AppState::create().await;

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
