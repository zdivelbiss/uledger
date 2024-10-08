#![allow(clippy::unused_unit)]

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
extern crate num_enum;

#[macro_use]
extern crate serde;

fn user_agent() -> &'static str {
    concat!("uledger-core/", env!("CARGO_PKG_VERSION"))
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "UPPERCASE")]
enum Kind {
    Equity,
    Asset,
    Liability,
    Income,
    Expense,
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
