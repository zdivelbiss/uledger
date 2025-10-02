#![allow(unused)]

use std::{net::SocketAddr, path::PathBuf, sync::LazyLock, time::Duration};
use tower_sessions::cookie::Key;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::registry::Data;

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    use figment::{Figment, providers::Env};

    Figment::new()
        .merge(Env::raw().split('_'))
        .extract()
        .expect("could not parse config")
});

#[derive(Debug, Deserialize)]
pub struct Config {
    pub network: Network,
    pub database: Database,
    pub session: Session,
    pub smtp: Smtp,
    pub log: Log,
}

#[derive(Debug, Deserialize)]
pub struct Network {
    pub bind: std::net::SocketAddr,

    #[serde(deserialize_with = "deserialize::duration::mils")]
    pub timeout: Duration,
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
pub struct Session {
    pub url: String,
    pub namespace: Option<u32>,

    #[serde(deserialize_with = "deserialize::duration::secs")]
    pub lifetime: Duration,

    #[serde(deserialize_with = "deserialize::base64::key")]
    pub key: Key,
}

#[derive(Debug, Deserialize)]
pub struct Smtp {
    pub user: String,
    pub pass: String,
    pub host: String,
    pub port: u16,
    pub from: String,
    pub replyto: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Log {
  pub  level: Option<String>
}

mod deserialize {
    pub mod base64 {
        use base64::{Engine, prelude::BASE64_STANDARD};
        use serde::{
            Deserializer,
            de::{Error, Unexpected, Visitor},
        };
        use std::{marker::PhantomData, time::Duration};
        use tower_sessions::cookie::Key;

        pub fn key<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Key, D::Error> {
            deserializer.deserialize_str(KeyVisitor)
        }

        pub struct KeyVisitor;

        impl Visitor<'_> for KeyVisitor {
            type Value = Key;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "expected a valid BASE64 string")
            }

            fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
                let mut slice = [0u8; 64];

                BASE64_STANDARD
                    .decode_slice(v, &mut slice)
                    .map_err(|error| {
                        Error::custom(format!("could not decode \"{v}\": {error:?}"))
                    })?;

                Ok(Key::from(&slice))
            }
        }
    }

    pub mod duration {
        use serde::{Deserializer, de::Visitor};
        use std::time::Duration;

        pub fn mils<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Duration, D::Error> {
            deserializer.deserialize_u32(SecsVisitor)
        }

        pub struct MilsVisitor;

        impl Visitor<'_> for MilsVisitor {
            type Value = Duration;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "expected a duration (in seconds)")
            }

            fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<Self::Value, E> {
                Ok(Duration::from_millis(v))
            }
        }

        pub fn secs<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Duration, D::Error> {
            deserializer.deserialize_u32(SecsVisitor)
        }

        pub struct SecsVisitor;

        impl Visitor<'_> for SecsVisitor {
            type Value = Duration;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "expected a duration (in seconds)")
            }

            fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<Self::Value, E> {
                Ok(Duration::from_secs(v))
            }
        }
    }
}
