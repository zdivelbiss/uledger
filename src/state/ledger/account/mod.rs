pub mod create;
pub mod delete;
pub mod read;
pub mod read_all;
pub mod update;

use crate::state::AppState;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug)]
pub struct AccountLedger {
    db: crate::Datastore,
}

impl axum::extract::FromRef<AppState> for AccountLedger {
    fn from_ref(app: &AppState) -> Self {
        Self { db: app.db.clone() }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountRecord {
    pub id: Uuid,
    pub created: DateTime<Utc>,
    pub name: String,
    pub description: Option<String>,
}
