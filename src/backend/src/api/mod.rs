use crate::{ledger::Ledger, sessions::Sessions};
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
use uuid::Uuid;

mod v1;

#[derive(Clone)]
pub struct State {
    pub sessions: Sessions<Uuid>,
    pub users: Users,
    pub ledger: Ledger
}

pub async fn accept_connections(listener: TcpListener, sessions: Sessions<Uuid>) -> Result<()> {
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

    let state = State { sessions };

    let app = Router::new()
        .layer(decompression_layer)
        .nest("/api/v1", v1::routes())
        .layer(set_server_layer)
        .layer(compression_layer)
        .with_state(state);

    info!("Begin listening for requests.");
    axum::serve(listener, app)
        .await
        .expect("error serving connections");

    Ok(())
}
