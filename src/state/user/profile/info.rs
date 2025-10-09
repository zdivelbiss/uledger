use crate::server::UserSession;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Database(#[from] sqlx::Error),
}

impl super::UserProfile {
    pub async fn get_display_name(&self, user_session: &UserSession) -> Result<String, Error> {
        let display_name = query!(
            "
            SELECT display_name
                FROM _user.profile
                WHERE
                    id = $1
                LIMIT 1
            ;
            ",
            user_session.id()
        )
        .fetch_one(&self.db)
        .await?;

        Ok(display_name.display_name)
    }
}
