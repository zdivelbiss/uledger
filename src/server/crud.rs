use crate::Datastore;

pub trait Create {
    type Args;
    type Error;

    async fn create(self, db: &Datastore, args: Self::Args) -> Result<(), Self::Error>;
}

pub trait Read {
    type Args;
    type Output;
    type Error;

    async fn read(self, db: &Datastore, args: Self::Args) -> Result<Self::Output, Self::Error>;
}

pub trait Update {
    type Args;
    type Error;

    async fn update(self, db: &Datastore, args: Self::Args) -> Result<(), Self::Error>;
}

pub trait Delete {
    type Args;
    type Error;

    async fn delete(self, db: &Datastore, args: Self::Args) -> Result<(), Self::Error>;
}
