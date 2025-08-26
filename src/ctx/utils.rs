use std::time::Duration;

use serde::{Deserialize, Deserializer};

pub fn is_running_under_systemd() -> bool {
    std::env::var("INVOCATION_ID").is_ok()
        || std::env::var("JOURNAL_STREAM").is_ok()
}
#[allow(unused)]
pub fn deserialize_duration<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: From<Duration> + Default
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(duration_str) => humantime::parse_duration(&duration_str)
            .map(T::from)
            .map_err(serde::de::Error::custom),
        None => Ok(T::default())
    }
}
