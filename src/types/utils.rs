use std::fmt::Display;

use serde::{Deserialize, Deserializer, Serializer};

pub fn to_string<S>(value: impl Display, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&value.to_string())
}

pub fn to_string_opt<S, T>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Display,
{
    match value {
        Some(v) => serializer.serialize_str(&v.to_string()),
        None => serializer.serialize_none(),
    }
}

pub fn as_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<f64>().map_err(serde::de::Error::custom)
}

pub fn as_f64_opt<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = Option::<String>::deserialize(deserializer)?;
    if let Some(s) = s {
        s.parse::<f64>().map(Some).map_err(serde::de::Error::custom)
    } else {
        Ok(None)
    }
}
