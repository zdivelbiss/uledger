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

mod v1;

pub async fn accept_connections(listener: TcpListener) -> Result<()> {
    trace!("Building API router...");

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
        .layer(compression_layer);

    info!("Begin listening for requests.");
    axum::serve(listener, app)
        .await
        .expect("error serving connections");

    Ok(())
}
