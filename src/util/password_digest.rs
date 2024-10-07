use base64::{prelude::BASE64_STANDARD_NO_PAD, Engine};
use serde::{
    de::{Unexpected, Visitor},
    Deserialize, Deserializer,
};

pub struct PasswordDigest([u8; 64]);

impl std::fmt::Debug for PasswordDigest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        BASE64_STANDARD_NO_PAD.encode(self.0).fmt(f)
    }
}

impl PasswordDigest {
    pub const fn as_slice(&self) -> &[u8] {
        &self.0
    }
}

impl<'de> Deserialize<'de> for PasswordDigest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(PasswordDigestVisitor)
    }
}

struct PasswordDigestVisitor;

impl<'de> Visitor<'de> for PasswordDigestVisitor {
    type Value = PasswordDigest;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a BASE64-encoded SHA512 digest")
    }

    fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
        let mut buf = [0u8; size_of::<PasswordDigest>()];

        BASE64_STANDARD_NO_PAD
            .decode_slice(v, &mut buf)
            .map_err(|_| E::invalid_value(Unexpected::Str(v), &self))?;

        Ok(PasswordDigest(buf))
    }
}
