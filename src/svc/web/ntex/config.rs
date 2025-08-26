use std::fmt;

use ntex::time::Seconds;
use serde::{
    de::{self, Deserializer},
    Deserialize
};
use serde_with::serde_as;

use crate::config::ServiceConfig;

#[serde_as]
#[derive(Deserialize)]
pub(crate) struct NtexConfig {
    pub max_conn: usize,

    pub max_conn_rate: usize,

    #[serde(deserialize_with = "de_seconds")]
    pub shutdown_timeout: Seconds,

    #[serde(deserialize_with = "de_seconds")]
    pub client_timeout: Seconds,

    #[serde(deserialize_with = "de_seconds")]
    pub disconnect_timeout: Seconds,

    pub headers_read_rate: HeaderReadRate,

    #[serde(deserialize_with = "de_seconds")]
    pub ssl_handshake_timeout: Seconds,

    #[serde(deserialize_with = "de_seconds")]
    pub keep_alive_interval: Seconds
}

#[derive(Deserialize)]
pub struct HeaderReadRate {
    #[serde(deserialize_with = "de_seconds")]
    pub timeout: Seconds,
    #[serde(deserialize_with = "de_seconds")]
    pub max_timeout: Seconds,
    pub rate: u16
}

impl ServiceConfig {
    pub(crate) fn ntex_config(&self) -> &NtexConfig {
        &self.ntex
    }
}

fn de_seconds<'de, D>(deserializer: D) -> Result<Seconds, D::Error>
where
    D: Deserializer<'de>
{
    struct SecondsVisitor;
    impl<'de> de::Visitor<'de> for SecondsVisitor {
        type Value = Seconds;

        fn expecting(
            &self,
            f: &mut fmt::Formatter
        ) -> fmt::Result {
            write!(f, "a human-readable duration string like '5s', '2m', '1h', '1d'")
        }

        fn visit_str<E>(
            self,
            s: &str
        ) -> Result<Self::Value, E>
        where
            E: de::Error
        {
            let s = s.trim();
            let d = humantime::parse_duration(s).map_err(E::custom)?;
            let secs = d.as_secs().min(u16::MAX as u64) as u16;
            Ok(Seconds::new(secs))
        }
    }
    deserializer.deserialize_any(SecondsVisitor)
}
