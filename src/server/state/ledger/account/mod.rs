pub mod create;
pub mod delete;
pub mod read;
pub mod update;

use crate::server::state::App;

#[derive(Debug)]
pub struct AccountLedger {
    db: crate::Datastore,
}

impl axum::extract::FromRef<App> for AccountLedger {
    fn from_ref(app: &App) -> Self {
        Self {
            db: app.db.clone(),
        }
    }
}
