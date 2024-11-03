use serde::de::{Error, Unexpected, Visitor};
use std::fmt;

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct VerificationToken([u8; Self::COMPELXITY]);

impl VerificationToken {
    pub const COMPELXITY: usize = 3;

    pub fn gen() -> Self {
        Self(rand::random())
    }

    pub fn from_str(v: impl AsRef<str>) -> Result<Self, hex::FromHexError> {
        let mut data = [0u8; Self::COMPELXITY];
        hex::decode_to_slice(v.as_ref(), &mut data)?;

        Ok(Self(data))
    }

    pub fn as_bytes(&self) -> &[u8; 3] {
        &self.0
    }
}

impl std::fmt::Display for VerificationToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        hex::encode_upper(self.0).fmt(f)
    }
}

// impl serde::Serialize for VerificationToken {
//     fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
//         serializer.serialize_str(&self.to_string())
//     }
// }

// impl<'de> serde::Deserialize<'de> for VerificationToken {
//     fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
//         deserializer.deserialize_str(VerificationTokenVisitor)
//     }
// }

struct VerificationTokenVisitor;

impl<'de> Visitor<'de> for VerificationTokenVisitor {
    type Value = VerificationToken;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a 3-byte hex string")
    }

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        VerificationToken::from_str(v).map_err(|_| Error::invalid_value(Unexpected::Str(v), &self))
    }
}
