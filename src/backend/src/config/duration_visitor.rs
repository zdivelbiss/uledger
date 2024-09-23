use serde::{de::Visitor, Deserializer};
use std::time::Duration;

pub struct DurationMilsVisitor;

impl Visitor<'_> for DurationMilsVisitor {
    type Value = Duration;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "expected a duration (in seconds)")
    }

    fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<Self::Value, E> {
        Ok(Duration::from_secs(v))
    }
}

pub fn deserialize_duration_mils<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Duration, D::Error> {
    deserializer.deserialize_u32(DurationSecsVisitor)
}

pub struct DurationSecsVisitor;

impl Visitor<'_> for DurationSecsVisitor {
    type Value = Duration;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "expected a duration (in seconds)")
    }

    fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<Self::Value, E> {
        Ok(Duration::from_millis(v))
    }
}

pub fn deserialize_duration_secs<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Duration, D::Error> {
    deserializer.deserialize_u32(DurationSecsVisitor)
}
