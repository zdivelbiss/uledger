use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("email address too long (max 128)")]
    EmailAddressLength,

    #[error("email address format is invalid")]
    EmailAddressFormat,

    #[error("display name too long (max 32)")]
    DisplayNameLength,

    #[error("user already exists")]
    DuplicateEmail,

    #[error(transparent)]
    Database(sqlx::Error),

    #[error(transparent)]
    PasswordHash(#[from] argon2::password_hash::Error),
}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        let Some(db_error) = error.as_database_error() else {
            return Self::Database(error);
        };

        match (db_error.code().as_deref(), db_error.constraint()) {
            (Some("23514"), Some("chk_email_address_len")) => Self::EmailAddressLength,
            (Some("23514"), Some("chk_email_address_format")) => Self::EmailAddressFormat,
            (Some("23514"), Some("profile_chk_display_name_len")) => Self::DisplayNameLength,
            (Some("23505"), Some("profile_unq_1" | "profile_unq_2")) => Self::DuplicateEmail,

            _ => Self::Database(error),
        }
    }
}

impl super::UserProfile {
    pub async fn register(
        &self,
        email_address: &str,
        password: &str,
        display_name: &str,
    ) -> Result<(), Error> {
        let salt_string = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt_string)?
            .serialize();

        query!(
            "
            INSERT INTO _user.profile
                    (email_address, password_salt, password_hash, display_name)
                VALUES
                    ($1, $2, $3, $4)
            ;
            ",
            email_address as _,
            salt_string.as_str() as _,
            password_hash.as_str() as _,
            display_name
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }
}
