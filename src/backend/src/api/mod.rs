use anyhow::Result;
use axum::{
    http::{header, HeaderValue},
    Router,
};
use tokio::net::TcpListener;
use tower_http::{
    compression::CompressionLayer, decompression::DecompressionLayer,
    set_header::SetResponseHeaderLayer,
};

mod state;
mod v1;

pub async fn init_state() -> Result<()> {
    state::init().await
}

pub async fn accept_connections() -> Result<()> {
    let state = state::get();

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
        HeaderValue::from_static(crate::agent_str()),
    );

    let app = Router::new()
        .layer(decompression_layer)
        .nest("/api/v1", v1::routes())
        .layer(set_server_layer)
        .layer(compression_layer)
        .with_state(state);

    let listener = bind_listener().await?;

    info!("Begin listening for requests.");
    axum::serve(listener, app)
        .await
        .expect("error serving connections");

    Ok(())
}

async fn bind_listener() -> Result<TcpListener> {
    let api_listener = tokio::time::timeout(
        std::time::Duration::from_secs(5),
        tokio::net::TcpListener::bind(crate::cfg().bind()),
    )
    .await??;

    Ok(api_listener)
}
