use crate::server::state::App;
use sqlx::{Pool, Postgres};

mod confirm;
pub use confirm::Error as ConfirmError;

mod create;
pub use create::Error as CreateError;

#[derive(Clone)]
pub struct UserVerification {
    db: Pool<Postgres>,
}

impl axum::extract::FromRef<App> for UserVerification {
    fn from_ref(app: &App) -> Self {
        Self {
            db: app.db().clone(),
        }
    }
}
