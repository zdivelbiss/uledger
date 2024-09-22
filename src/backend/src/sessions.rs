use crate::api::State;
use anyhow::Result;
use redis::{
    aio::MultiplexedConnection, AsyncCommands, Client, FromRedisValue, IntoConnectionInfo,
    ToRedisArgs,
};
use std::marker::PhantomData;
use uuid::Uuid;

#[derive(Clone)]
pub struct Sessions<T>(MultiplexedConnection, PhantomData<T>);

impl<T> Sessions<T> {
    pub async fn connect(url: impl IntoConnectionInfo) -> Result<Self> {
        let client = Client::open(url).unwrap();
        let connection = client.get_multiplexed_async_connection().await?;

        Ok(Self(connection, PhantomData))
    }
}

impl<T: ToRedisArgs + Send + Sync> Sessions<T> {
    pub async fn gen_session(&mut self, data: T) -> Result<Uuid> {
        let id = Uuid::now_v7();

        let _: () = self.0.set(id, data).await?;

        Ok(id)
    }
}

impl<T: FromRedisValue + Sync + Send> Sessions<T> {
    pub async fn get_session(&mut self, id: Uuid) -> Result<Option<T>> {
        let opt = self.0.get(id).await?;

        Ok(opt)
    }
}

impl axum::extract::FromRef<State> for Sessions<Uuid> {
    fn from_ref(input: &State) -> Self {
        input.sessions.clone()
    }
}
