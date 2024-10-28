use crate::server::state::App;
use sqlx::{Pool, Postgres};

pub mod login;
pub mod register;

#[derive(Debug, Clone)]
pub struct UserAccounts {
    db: Pool<Postgres>,
}

impl UserAccounts {
    fn db(&self) -> &Pool<Postgres> {
        &self.db
    }
}

impl axum::extract::FromRef<App> for UserAccounts {
    fn from_ref(app_state: &App) -> Self {
        Self {
            db: app_state.db.clone(),
        }
    }
}

