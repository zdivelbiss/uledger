#![allow(clippy::unused_unit)]
#![deny(
    clippy::disallowed_types,
    clippy::disallowed_macros,
    clippy::disallowed_methods,
    clippy::disallowed_names
)]

mod postmark;
mod server;

mod util;
use util::*;

mod config;
use config::cfg;

#[macro_use]
extern crate tracing;

#[macro_use]
extern crate sqlx;

#[macro_use]
extern crate strum;

#[macro_use]
extern crate serde;

const fn user_agent() -> &'static str {
    concat!("uledger-srv/", env!("CARGO_PKG_VERSION"))
}

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    {
        dotenvy::dotenv().expect("no `.env` file");

        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();
    }
    #[cfg(not(debug_assertions))]
    {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .init();
    }

    server::run().await;

    info!("Reached safe shutdown point.");
}
