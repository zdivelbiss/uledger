use super::EmailAddress;
use sqlx::{
    encode::IsNull, error::BoxDynError, postgres::PgTypeInfo, Database, Encode, Postgres, Type,
};

impl Type<Postgres> for EmailAddress {
    fn type_info() -> <Postgres as Database>::TypeInfo {
        PgTypeInfo::with_name("TEXT")
    }
}

impl Encode<'_, Postgres> for EmailAddress {
    fn encode_by_ref(
        &self,
        buf: &mut <Postgres as Database>::ArgumentBuffer<'_>,
    ) -> Result<IsNull, BoxDynError> {
        buf.extend_from_slice(self.as_str().as_bytes());

        Ok(IsNull::No)
    }

    fn encode(
        self,
        buf: &mut <Postgres as Database>::ArgumentBuffer<'_>,
    ) -> Result<IsNull, BoxDynError>
    where
        Self: Sized,
    {
        self.encode_by_ref(buf)
    }

    fn size_hint(&self) -> usize {
        self.as_str().len()
    }
}
