use redis::ToRedisArgs;
use regex::Regex;
use serde::{de::Visitor, Deserialize, Serialize};
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

impl From<EmailAddress> for String {
    fn from(value: EmailAddress) -> Self {
        value.0
    }
}

impl Serialize for EmailAddress {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

struct DeserializeEmailAddressVisitor;

impl<'de> Visitor<'de> for DeserializeEmailAddressVisitor {
    type Value = EmailAddress;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a well-formatted email address")
    }

    fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        use serde::de::{Error, Unexpected};

        EmailAddress::new(v).map_err(|v| Error::invalid_value(Unexpected::Str(&v), &self))
    }

    fn visit_string<E>(self, v: String) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        use serde::de::{Error, Unexpected};

        EmailAddress::new(&v).map_err(|v| Error::invalid_value(Unexpected::Str(&v), &self))
    }
}

impl<'de> Deserialize<'de> for EmailAddress {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_string(DeserializeEmailAddressVisitor)
    }
}

impl ToRedisArgs for EmailAddress {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        ToRedisArgs::write_redis_args(&self.0, out);
    }
}
