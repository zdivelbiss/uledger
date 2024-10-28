use regex::Regex;
use serde::{
    de::{self, Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};
use std::fmt;
use std::sync::LazyLock;

static EMAIL_ADDRESS_VALIDATOR: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#).expect("regex invalid")
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmailAddress(String);

impl EmailAddress {
    pub fn new<Str: AsRef<str>>(email_address: Str) -> std::result::Result<Self, Str> {
        if EMAIL_ADDRESS_VALIDATOR.is_match(email_address.as_ref()) {
            Ok(Self(email_address.as_ref().to_string()))
        } else {
            Err(email_address)
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl core::fmt::Display for EmailAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Serialize for EmailAddress {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for EmailAddress {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_string(DeserializeEmailAddressVisitor)
    }
}

struct DeserializeEmailAddressVisitor;

impl<'de> de::Visitor<'de> for DeserializeEmailAddressVisitor {
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
