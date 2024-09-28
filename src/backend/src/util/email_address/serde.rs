use serde::{
    de::{self, Visitor},
    Deserializer, Serialize, Serializer,
};
use std::fmt;

use super::EmailAddress;

impl Serialize for EmailAddress {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> serde::Deserialize<'de> for EmailAddress {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_string(DeserializeEmailAddressVisitor)
    }
}

struct DeserializeEmailAddressVisitor;

impl<'de> Visitor<'de> for DeserializeEmailAddressVisitor {
    type Value = EmailAddress;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a well-formatted email address")
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        EmailAddress::new(v).map_err(|v| de::Error::invalid_value(de::Unexpected::Str(v), &self))
    }

    fn visit_string<E: de::Error>(self, v: String) -> Result<Self::Value, E> {
        EmailAddress::new(&v).map_err(|v| de::Error::invalid_value(de::Unexpected::Str(v), &self))
    }
}
