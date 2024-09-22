use crate::email_address::EmailAddress;
use serde::{de::Visitor, Deserialize, Deserializer};
use std::{net::SocketAddr, sync::LazyLock, time::Duration};

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
    bind: std::net::SocketAddr,

    database_url: String,
    database_pool_size: u32,

    sessions_url: String,
    sessions_db_num: Option<u32>,
    #[serde(deserialize_with = "deserialize_duration_secs")]
    sessions_lifetime: Duration,

    verifications_url: String,
    verifications_db_num: Option<u32>,
    #[serde(deserialize_with = "deserialize_duration_secs")]
    verifications_lifetime: Duration,

    postmark_api_key: String,
    postmark_from_address: EmailAddress,
}

impl Config {
    pub fn bind(&self) -> SocketAddr {
        self.bind
    }

    pub fn database_url(&self) -> &str {
        &self.database_url
    }

    pub fn database_pool_size(&self) -> u32 {
        self.database_pool_size
    }

    pub fn sessions_url(&self) -> &str {
        &self.sessions_url
    }

    pub fn sessions_db_num(&self) -> Option<u32> {
        self.sessions_db_num
    }

    pub fn sessions_lifetime(&self) -> Duration {
        self.sessions_lifetime
    }

    pub fn verifications_url(&self) -> &str {
        &self.verifications_url
    }

    pub fn verifications_db_num(&self) -> Option<u32> {
        self.verifications_db_num
    }

    pub fn verifications_lifetime(&self) -> Duration {
        self.verifications_lifetime
    }

    pub fn postmark_api_key(&self) -> &str {
        &self.postmark_api_key
    }
}

struct DurationSecsVisitor;

impl Visitor<'_> for DurationSecsVisitor {
    type Value = Duration;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "expected a duration (in seconds)")
    }

    fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<Self::Value, E> {
        Ok(Duration::from_secs(v))
    }
}

fn deserialize_duration_secs<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> core::result::Result<Duration, D::Error> {
    deserializer.deserialize_u32(DurationSecsVisitor)
}
