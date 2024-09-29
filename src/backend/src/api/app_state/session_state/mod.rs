use crate::api::app_state::AppState;
use redis::{aio::MultiplexedConnection, cmd, AsyncCommands, Client, RedisResult};
use uuid::Uuid;

mod session;
pub use session::*;

#[derive(Clone)]
pub struct SessionState(MultiplexedConnection);

impl SessionState {
    pub async fn connect() -> RedisResult<Self> {
        use crate::cfg;

        let url = cfg().session.url.as_str();
        debug!("Connecting to session storage: {url:?}");

        let client = Client::open(url)?;
        let mut conn = client.get_multiplexed_async_connection().await?;

        if let Some(namespace) = cfg().session.namespace {
            cmd("SELECT").arg(namespace).exec_async(&mut conn).await?;
        }

        Ok(Self(conn))
    }

    fn get_connection(&self) -> MultiplexedConnection {
        self.0.clone()
    }

    #[instrument(skip(self))]
    pub async fn store(&self, session: Session) -> RedisResult<Uuid> {
        let mut conn = self.get_connection();

        let token = Uuid::now_v7();
        let session_json = serde_json::to_string(&session).unwrap();
        let session_lifetime = crate::cfg().session.lifetime.as_secs();

        cmd("SET")
            .arg(token)
            .arg(session_json)
            .arg("EX")
            .arg(session_lifetime)
            .arg("NX")
            .exec_async(&mut conn)
            .await?;

        Ok(token)
    }

    pub async fn get(&self, token: Uuid) -> RedisResult<Option<Session>> {
        let mut conn = self.get_connection();

        let session_json = cmd("GET")
            .arg(token)
            .query_async::<String>(&mut conn)
            .await?;

        let session = serde_json::from_str(&session_json).unwrap();

        Ok(session)
    }
}

impl axum::extract::FromRef<super::AppState> for SessionState {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.session_state().clone()
    }
}
