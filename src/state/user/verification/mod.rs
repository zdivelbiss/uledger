use crate::state::AppState;
use sqlx::{Pool, Postgres};

pub mod confirm;
pub mod create;

#[derive(Clone)]
pub struct UserVerification {
    db: Pool<Postgres>,
}

impl axum::extract::FromRef<AppState> for UserVerification {
    fn from_ref(app: &AppState) -> Self {
        Self { db: app.db.clone() }
    }
}
