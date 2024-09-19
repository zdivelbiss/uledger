use std::sync::LazyLock;

mod config;
mod ledger;

#[macro_use]
extern crate tracing;

static CFG: LazyLock<config::Config> = LazyLock::new(|| {
    use figment::*;

    Figment::new()
        .merge(figment::providers::Env::prefixed("ACCLE_"))
        .extract()
        .expect("could not parse config")
});

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

    let client = ledger::Ledger::open(
        &CFG.postgrest_endpoint,
        &CFG.postgrest_apikey,
        CFG.postgrest_servicekey.as_deref(),
    );

    let accounts = client.get_accounts().await?;

    info!("{accounts:?}");

    Ok(())
}
