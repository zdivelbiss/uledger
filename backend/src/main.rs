use std::sync::LazyLock;

use sqlx::postgres::PgPoolOptions;

mod config;
mod ledger;

#[macro_use]
extern crate tracing;

static CFG: LazyLock<config::Config> = LazyLock::new(|| {
    use figment::*;

    Figment::new()
        .merge(figment::providers::Env::raw())
        .extract()
        .expect("could not parse config")
});

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!();

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
        .max_connections(CFG.database_pool_size)
        .connect(&CFG.database_url)
        .await?;

    MIGRATOR.run(&pool).await?;

    let result = sqlx::query!("SELECT table_name FROM information_schema.tables WHERE table_type = 'BASE TABLE' AND table_schema NOT IN ('pg_catalog', 'information_schema');").fetch_all(&pool).await?;

    info!("{result:?}");

    Ok(())
}
