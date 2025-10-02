use regex::Regex;
use serde::{
    de::{self, Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};
use std::fmt;
use std::sync::LazyLock;

static EMAIL_ADDRESS_VALIDATOR: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$"#).unwrap()
});

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "EMAIL_ADDRESS")]
pub struct EmailAddress(String);

impl EmailAddress {
    pub fn new<Str: AsRef<str>>(email_address: Str) -> std::result::Result<Self, Str> {
        if EMAIL_ADDRESS_VALIDATOR.is_match(email_address.as_ref()) {
            Ok(Self(email_address.as_ref().to_owned()))
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
