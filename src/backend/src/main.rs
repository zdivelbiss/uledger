#![allow(clippy::unused_unit)]

mod postmark;
mod util;
mod api;

mod config;
use config::cfg;

#[macro_use]
extern crate tracing;

#[macro_use]
extern crate sqlx;

fn user_agent() -> &'static str {
    concat!("uledger-core/", env!("CARGO_PKG_VERSION"))
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

    api::run_server().await;

    info!("Reached safe shutdown point.");
}
