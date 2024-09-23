use config::cfg;

mod api;
mod config;
// mod ledger;
mod email;
mod util;

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
            .with_max_level(tracing::Level::TRACE)
            .init();
    }
    #[cfg(not(debug_assertions))]
    {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .init();
    }

    api::init_state().await;
    api::accept_connections().await;

    info!("Reached safe shutdown point.");
}
