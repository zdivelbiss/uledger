use crate::server::state::App;
use sqlx::{Pool, Postgres};

mod login;
pub use login::Error as LoginError;

mod register;
pub use register::Error as RegisterError;

#[derive(Clone)]
pub struct UserAccount {
    db: Pool<Postgres>,
}

impl axum::extract::FromRef<App> for UserAccount {
    fn from_ref(app: &App) -> Self {
        Self {
            db: app.db().clone(),
        }
    }
}
