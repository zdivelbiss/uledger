use crate::api::app_state::AppState;
use redis::{aio::MultiplexedConnection, cmd, AsyncCommands, Client, RedisResult};
use std::time::Duration;
use uuid::Uuid;

#[derive(Clone)]
pub struct SessionState(MultiplexedConnection);

impl SessionState {
    pub async fn connect() -> RedisResult<Self> {
        use crate::cfg;

        let url = cfg().session.url.as_str();
        let client = Client::open(url)?;
        let mut connection = client.get_multiplexed_async_connection().await?;

        if let Some(namespace) = cfg().session.namespace {
            cmd("SELECT")
                .arg(namespace)
                .exec_async(&mut connection)
                .await?;
        }

        Ok(Self(connection))
    }

    fn connection(&mut self) -> &mut MultiplexedConnection {
        &mut self.0
    }

    pub async fn create(&mut self, id: Uuid, lifetime: Duration) -> RedisResult<Uuid> {
        let token = Uuid::now_v7();

        cmd("SET")
            .arg(id)
            .arg(token)
            .arg("EX")
            .arg(lifetime.as_secs())
            .arg("NX")
            .exec_async(self.connection())
            .await?;

        Ok(token)
    }

    pub async fn get(&mut self, id: Uuid) -> RedisResult<Option<Uuid>> {
        self.connection().get(id).await
    }
}

impl axum::extract::FromRef<super::AppState> for SessionState {
    fn from_ref(state: &AppState) -> Self {
        state.1.clone()
    }
}
