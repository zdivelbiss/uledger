#[derive(Debug, serde::Deserialize)]
pub struct Config {
    pub database_url: String,
    pub database_pool_size: u32,
}
