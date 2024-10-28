use sqlx::{Pool, Postgres};

pub struct Ledger {
    db: Pool<Postgres>,
}

impl Ledger {
    fn db(&self) -> &Pool<Postgres> {
        &self.db
    }
}

impl axum::extract::FromRef<super::App> for Ledger {
    fn from_ref(app_state: &super::App) -> Self {
        Self {
            db: app_state.db.clone(),
        }
    }
}
