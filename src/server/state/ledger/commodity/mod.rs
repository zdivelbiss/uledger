pub mod create;
pub mod delete;
pub mod read;
pub mod read_all;
pub mod update;

use crate::server::state::App;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct CommodityRecord {
    pub id: Uuid,
    pub created: DateTime<Utc>,
    pub name: String,
    pub format: String,
}

#[derive(Debug)]
pub struct CommodityLedger {
    db: crate::Datastore,
}

impl axum::extract::FromRef<App> for CommodityLedger {
    fn from_ref(app: &App) -> Self {
        Self { db: app.db.clone() }
    }
}
