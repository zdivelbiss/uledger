use crate::server::crud::{Create, Delete, Read, Update};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub mod account;

pub struct Ledger {
    db: Pool<Postgres>,
    user_id: Uuid,
}

impl Ledger {
    pub async fn create<T: Create<Args = Uuid>>(&self, entry: T) -> Result<(), T::Error> {
        entry.create(&self.db, self.user_id).await
    }
    
    pub async fn read<T: Read<Args = Uuid>>(&self, entry: T) -> Result<T::Output, T::Error> {
        entry.read(&self.db, self.user_id).await
    }

    pub async fn update<T: Update<Args = Uuid>>(&self, entry: T) -> Result<(), T::Error> {
        entry.update(&self.db, self.user_id).await
    }

    pub async fn delete<T: Delete<Args = Uuid>>(&self, entry: T) -> Result<(), T::Error> {
        entry.delete(&self.db, self.user_id).await
    }
}
