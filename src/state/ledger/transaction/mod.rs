pub mod create;
pub mod delete;
pub mod read;
pub mod read_all;
pub mod update;

use crate::state::AppState;
use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;

#[derive(Debug)]
pub struct TransactionLedger {
    db: crate::Datastore,
}

impl axum::extract::FromRef<AppState> for TransactionLedger {
    fn from_ref(app: &AppState) -> Self {
        Self { db: app.db.clone() }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionRecord {
    pub id: Uuid,
    pub created: DateTime<Utc>,
    pub occurred_on: NaiveDate,
    pub account: Uuid,
    pub payee: Uuid,
    pub currency: String,
    pub amount: f64,
    pub description: Option<String>,
}
