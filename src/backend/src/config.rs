use std::net::SocketAddr;

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    pub database_url: String,
    pub database_pool_size: u32,
    pub bind_address: SocketAddr,
}
