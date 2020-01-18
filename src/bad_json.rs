//! EFT API returns a lot of inconsistent and bad JSON. Serde deserializers to fix those broken JSON.

use serde::de::Visitor;
use serde::{Deserialize, Deserializer};
use serde_json::Value;
use std::fmt;

fn deserialize_integer_to_string<'de, D>(de: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let json: serde_json::Value = Deserialize::deserialize(de)?;
    match json {
        Value::Number(i) => Ok(i.to_string()),
        _ => Err(serde::de::Error::custom("Unexpected value")),
    }
}

fn deserialize_empty_as_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = match String::deserialize(deserializer) {
        Ok(s) => s,
        Err(_) => {
            return Ok(None);
        }
    };

    if s.is_empty() {
        Ok(None)
    } else {
        Ok(Some(s))
    }
}

#[derive(Debug, Clone, PartialEq)]
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
