pub mod create;
pub mod delete;
pub mod read;
pub mod read_all;
pub mod update;

use crate::state::App;
use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConversionRecord {
    pub id: Uuid,
    pub created: DateTime<Utc>,
    pub effective: NaiveDate,
    pub from_commodity: Uuid,
    pub to_commodity: Uuid,
    pub rate: f64,
}

#[derive(Debug)]
pub struct ConversionLedger {
    db: crate::Datastore,
}

impl axum::extract::FromRef<App> for ConversionLedger {
    fn from_ref(app: &App) -> Self {
        Self { db: app.db.clone() }
    }
}
