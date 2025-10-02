use crate::state::App;
use sqlx::{Pool, Postgres};

pub mod confirm;
pub mod create;

#[derive(Clone)]
pub struct UserVerification {
    db: Pool<Postgres>,
}

impl axum::extract::FromRef<App> for UserVerification {
    fn from_ref(app: &App) -> Self {
        Self { db: app.db.clone() }
    }
}
