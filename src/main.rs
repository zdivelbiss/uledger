#![allow(clippy::unused_unit, clippy::too_many_arguments)]
#![deny(
    clippy::disallowed_types,
    clippy::disallowed_macros,
    clippy::disallowed_methods,
    clippy::disallowed_names
)]

mod postmark;
mod server;

mod config;
use config::cfg;

mod util;
use util::*;

#[macro_use]
extern crate tracing;

#[macro_use]
extern crate sqlx;

#[macro_use]
extern crate serde;


pub type Datastore = sqlx::Pool<sqlx::Postgres>;

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

    let currency = Commodity::from(CurrencyKind::JPY);
    let formatted = currency.parse(123_456_789_0000);

    info!("{formatted}");
    std::process::exit(0);

    server::run().await;

    info!("Reached safe shutdown point.");
}
