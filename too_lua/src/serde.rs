use serde::Deserialize as _;
use std::{borrow::Cow, str::FromStr};

pub fn from_str<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: FromStr,
    T::Err: std::fmt::Display,
{
    <Cow<'_, str>>::deserialize(deserializer)?
        .parse::<T>()
        .map_err(serde::de::Error::custom)
}
