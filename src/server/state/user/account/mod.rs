use crate::server::state::App;
use sqlx::{Pool, Postgres};

pub mod login;
pub mod register;

#[derive(Clone)]
pub struct UserAccount {
    db: Pool<Postgres>,
}

impl axum::extract::FromRef<App> for UserAccount {
    fn from_ref(app: &App) -> Self {
        Self { db: app.db.clone() }
    }
}
