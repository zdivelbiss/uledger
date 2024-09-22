use config::cfg;
use sqlx::postgres::PgPoolOptions;

mod api;
mod config;
mod ledger;
mod sessions;
mod users;

#[macro_use]
extern crate tracing;

#[macro_use]
extern crate sqlx;

static MIGRATOR: sqlx::migrate::Migrator = migrate!();

fn agent_str() -> &'static str {
    concat!("uledger-core/", env!("CARGO_PKG_VERSION"))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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

    let pool = PgPoolOptions::new()
        .max_connections(cfg().database_pool_size())
        .connect(cfg().database_url())
        .await?;
    MIGRATOR.run(&pool).await?;

    let sessions = sessions::Sessions::<uuid::Uuid>::connect(cfg().sessions_url()).await?;

    let api_listener = tokio::time::timeout(
        std::time::Duration::from_secs(5),
        tokio::net::TcpListener::bind(cfg().bind()),
    )
    .await??;

    api::accept_connections(api_listener, sessions).await?;

    Ok(())
}
