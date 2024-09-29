#![allow(unused)]

use crate::util::EmailAddress;
use std::{net::SocketAddr, sync::LazyLock, time::Duration};

mod duration_visitor;
use duration_visitor::*;
use serde::Deserialize;
use tracing_subscriber::registry::Data;

static CFG: LazyLock<Config> = LazyLock::new(|| {
    use figment::{providers::Env, Figment};

    Figment::new()
        .merge(Env::raw().split('_'))
        .extract()
        .expect("could not parse config")
});

pub fn cfg() -> &'static Config {
    &CFG
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub network: Network,
    pub apikey: ApiKey,
    pub database: Database,
    pub session: Storage,
    pub postmark: Postmark,
}

#[derive(Debug, Deserialize)]
pub struct Network {
    pub bind: std::net::SocketAddr,

    #[serde(deserialize_with = "deserialize_duration_mils")]
    pub timeout: Duration,
}

#[derive(Debug, Deserialize)]
pub struct ApiKey {
    pub cookies: String,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
    pub pool: Pool,
}

#[derive(Debug, Deserialize)]
pub struct Pool {
    pub size: u32,
}

#[derive(Debug, Deserialize)]
pub struct Storage {
    pub url: String,
    pub namespace: Option<u32>,

    #[serde(deserialize_with = "deserialize_duration_secs")]
    pub lifetime: Duration,
}

#[derive(Debug, Deserialize)]
pub struct Postmark {
    pub apikey: String,
    pub sender: EmailAddress,
}
