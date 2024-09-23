use crate::util::EmailAddress;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("user with email already exists")]
    UserExists,

    #[error("unknown error")]
    Unknown,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Role {
    Admin,
    Regular,
}

impl From<Role> for &'static str {
    fn from(value: Role) -> Self {
        match value {
            Role::Admin => "ADM",
            Role::Regular => "REG",
        }
    }
}

#[derive(Clone)]
pub struct UserState(Pool<Postgres>);

impl UserState {
    fn pool(&self) -> &Pool<Postgres> {
        &self.0
    }

    pub fn new(db: Pool<Postgres>) -> Self {
        Self(db)
    }

    pub async fn register_user(
        &self,
        email_address: &EmailAddress,
        password_digest: &[u8; 512],
    ) -> Result<Uuid> {
        use argon2::{
            password_hash::{rand_core::OsRng, SaltString},
            Argon2, PasswordHasher,
        };

        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(password_digest, &salt)
            .unwrap()
            .serialize();

        let role_str: &'static str = Role::Regular.into();
        let email_address_str = email_address.as_str();
        let salt_str = salt.as_str();
        let password_hash_str = password_hash.as_str();

        let result = query!(
            "INSERT INTO auth.users (role, email, salt, password_hash) VALUES ($1, $2, $3, $4) RETURNING auth.users.id;",
            role_str,
            email_address_str,
            salt_str,
            password_hash_str
        ).fetch_one(self.pool())
        .await;

        result.map(|r| r.id).map_err(|err| {
            let Some(err) = err.as_database_error() else {
                return Error::Unknown;
            };
            let Some(err_code) = err.code() else {
                return Error::Unknown;
            };

            match &*err_code {
                "23505" if err.constraint() == Some("users_email_key") => Error::UserExists,
                _ => Error::Unknown,
            }
        })
    }
}

impl axum::extract::FromRef<super::AppState> for UserState {
    fn from_ref(state: &super::AppState) -> Self {
        state.0.clone()
    }
}
