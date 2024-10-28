use sqlx::{Pool, Postgres};

pub mod login;
pub mod register;

#[derive(Debug, Clone)]
pub struct User {
    db: Pool<Postgres>,
}

impl User {
    fn db(&self) -> &Pool<Postgres> {
        &self.db
    }
}

impl axum::extract::FromRef<super::App> for User {
    fn from_ref(app_state: &super::App) -> Self {
        Self {
            db: app_state.db.clone(),
        }
    }
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "UPPERCASE")]
#[sqlx(type_name = "USER_ACCESS", rename_all = "UPPERCASE")]
pub enum UserAccess {
    Admin,
    Regular,
}
