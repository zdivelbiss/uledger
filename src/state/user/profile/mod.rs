use crate::state::AppState;
use sqlx::{Pool, Postgres};

pub mod login;
pub mod register;

#[derive(Clone)]
pub struct UserProfile {
    db: Pool<Postgres>,
}

impl axum::extract::FromRef<AppState> for UserProfile {
    fn from_ref(app: &AppState) -> Self {
        Self { db: app.db.clone() }
    }
}
