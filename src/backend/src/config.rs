use serde::Deserialize;
use std::{net::SocketAddr, sync::LazyLock};

static CFG: LazyLock<Config> = LazyLock::new(|| {
    use figment::{providers::Env, Figment};

    Figment::new()
        .merge(Env::raw().split('_'))
        .extract()
        .expect("could not parse config")
});

pub fn cfg() -> &'static Config {
    &*CFG
}

#[derive(Debug, Deserialize)]
pub struct Config {
    bind: std::net::SocketAddr,
    database: Database,
    sessions: Sessions,
}

#[derive(Debug, Deserialize)]
struct Database {
    url: String,
    pool: Pool,
}

#[derive(Debug, Deserialize)]
struct Pool {
    size: u32,
}

#[derive(Debug, Deserialize)]
struct Sessions {
    url: String,
}

impl Config {
    pub fn bind(&self) -> SocketAddr {
        self.bind
    }

    pub fn database_url(&self) -> &str {
        &self.database.url
    }

    pub fn database_pool_size(&self) -> u32 {
        self.database.pool.size
    }

    pub fn sessions_url(&self) -> &str {
        &self.sessions.url
    }
}
