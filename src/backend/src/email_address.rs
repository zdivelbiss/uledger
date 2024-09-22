use redis::ToRedisArgs;
use regex::Regex;
use reqwest::header::HeaderMap;
use serde::{de::Visitor, Deserialize, Serialize};
use std::sync::LazyLock;

#[derive(Debug, Serialize)]
enum MessageStream {
    Verification,
}

static HEADERS: LazyLock<HeaderMap> = LazyLock::new(|| {
    let mut header_map = HeaderMap::new();

    header_map.insert("Accept", "application/json".try_into().unwrap());
    header_map.insert("Content-Type", "application/json".try_into().unwrap());
    header_map.insert(
        "X-Postmark-Server-Token",
        crate::cfg().postmark_api_key().try_into().unwrap(),
    );

    header_map
});

#[derive(Debug, Serialize)]
struct Email {
    #[serde(rename = "From")]
    from: String,

    #[serde(rename = "To")]
    to: String,

    #[serde(rename = "Subject")]
    subject: String,

    #[serde(rename = "HtmlBody")]
    html_body: String,

    #[serde(rename = "MessageStream")]
    message_stream: MessageStream,
}

static EMAIL_ADDRESS_VALIDATOR: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#).expect("regex invalid")
});

#[derive(Debug)]
pub struct EmailAddress(String);

impl EmailAddress {
    pub fn new(email_address: impl AsRef<str>) -> Option<Self> {
        EMAIL_ADDRESS_VALIDATOR
            .is_match(email_address.as_ref())
            .then_some(Self(email_address.as_ref().to_string()))
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

impl Visitor<'_> for DeserializeEmailAddressVisitor {
    type Value = EmailAddress;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "expected a valid email address")
    }

    fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Self::visit_string(self, v.to_string())
    }

    fn visit_string<E>(self, v: String) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if EMAIL_ADDRESS_VALIDATOR.is_match(v.as_str()) {
            Ok(EmailAddress(v))
        } else {
            Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(&v),
                &self,
            ))
        }
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
        <String as ToRedisArgs>::write_redis_args(&self.0, out);
    }
}
