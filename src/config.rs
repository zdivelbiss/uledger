#![allow(unused)]

use lib::EmailAddress;
use std::{net::SocketAddr, path::PathBuf, sync::LazyLock, time::Duration};
use tower_sessions::cookie::Key;
use tracing_subscriber::registry::Data;

pub fn cfg() -> &'static Config {
    static CFG: LazyLock<Config> = LazyLock::new(|| {
        use figment::{providers::Env, Figment};

        Figment::new()
            .merge(Env::raw().split('_'))
            .extract()
            .expect("could not parse config")
    });

    &CFG
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub network: Network,
    pub database: Database,
    pub session: Session,
    pub postmark: Postmark,
    pub assets: Assets,
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
pub struct Postmark {
    pub apikey: String,
    pub sender: EmailAddress,
}

#[derive(Debug, Deserialize)]
pub struct Assets {
    pub path: PathBuf,
    pub cache: Cache,
}

#[derive(Debug, Deserialize)]
pub struct Cache {
    pub capacity: u64,

    #[serde(deserialize_with = "deserialize::duration::secs")]
    pub lifetime: Duration,
}

mod deserialize {
    pub mod base64 {
        use base64::{prelude::BASE64_STANDARD_NO_PAD, Engine};
        use serde::{
            de::{Error, Unexpected, Visitor},
            Deserializer,
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

                BASE64_STANDARD_NO_PAD
                    .decode_slice(v, &mut slice)
                    .map_err(|_| Error::invalid_value(Unexpected::Str(v), &self))?;

                Ok(Key::from(&slice))
            }
        }
    }

    pub mod duration {
        use serde::{de::Visitor, Deserializer};
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
