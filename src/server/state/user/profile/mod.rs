use crate::server::state::App;
use sqlx::{Pool, Postgres};

pub mod login;
pub mod register;

#[derive(Clone)]
pub struct UserProfile {
    db: Pool<Postgres>,
}

impl axum::extract::FromRef<App> for UserProfile {
    fn from_ref(app: &App) -> Self {
        Self { db: app.db.clone() }
    }
}
