#![allow(clippy::unused_unit, clippy::too_many_arguments)]
#![deny(
    clippy::disallowed_types,
    clippy::disallowed_macros,
    clippy::disallowed_methods,
    clippy::disallowed_names
)]

mod config;
mod email;
mod server;
mod state;
mod util;

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
    dotenvy::dotenv().expect("no `.env` file");

    init_tracing();
    email::init_smtp_transport();
    server::run().await;

    info!("Reached safe shutdown point.");
}

fn init_tracing() {
    let log_level_filter = config::CONFIG.log.level.as_ref().map_or(
        tracing::level_filters::LevelFilter::INFO,
        |level_filter| {
            level_filter
                .parse::<tracing::level_filters::LevelFilter>()
                .expect("failed to parse `LOG_LEVEL`")
        },
    );

    tracing_subscriber::fmt()
        .with_max_level(log_level_filter)
        .init();
}
