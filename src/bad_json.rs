//! EFT API returns a lot of inconsistent and bad JSON. Serde deserializers to fix those broken JSON.

use crate::inventory::Location;
use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::fmt;

pub(crate) fn deserialize_integer_to_string<'de, D>(de: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let json: serde_json::Value = Deserialize::deserialize(de)?;
    match json {
        Value::Number(i) => Ok(i.to_string()),
        Value::String(s) => Ok(s),
        _ => Err(Error::custom("Unexpected value")),
    }
}

pub(crate) fn deserialize_integer_to_option_string<'de, D>(
    de: D,
) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let json: serde_json::Value = Deserialize::deserialize(de)?;
    match json {
        Value::Number(i) => Ok(Some(i.to_string())),
        Value::String(s) => Ok(Some(s)),
        _ => Ok(None),
    }
}

pub(crate) fn deserialize_bad_location_as_none<'de, D>(de: D) -> Result<Option<Location>, D::Error>
where
    D: Deserializer<'de>,
{
    let json: serde_json::Value = Deserialize::deserialize(de)?;
    match json {
        Value::Object(_) => Ok(Some(
            serde_json::from_value(json).map_err(|_| Error::custom("Unexpected value"))?,
        )),
        _ => Ok(None),
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct StringOrInt(String);

impl<'de> Deserialize<'de> for StringOrInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StringOrIntVisitor;

        impl<'de> Visitor<'de> for StringOrIntVisitor {
            type Value = StringOrInt;

            fn expecting(&self, w: &mut fmt::Formatter) -> fmt::Result {
                write!(w, "string or integer represented as String.")
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(StringOrInt(v.to_string()))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(StringOrInt(v.to_string()))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(StringOrInt(v.to_string()))
            }
        }

        deserializer.deserialize_any(StringOrIntVisitor)
    }
}

impl StringOrInt {
    pub fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
