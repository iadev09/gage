use std::time::Duration;

use serde::Deserialize;
use serde_with::serde_as;

use crate::config::ServiceConfig;
use crate::ctx::utils::deserialize_duration;

#[serde_as]
#[derive(Deserialize)]
pub(crate) struct HyperConfig {
    pub max_concurrent_streams: Option<u32>,

    #[serde(deserialize_with = "deserialize_duration")]
    pub idle_timeout: Duration,

    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_duration")]
    pub graceful_timeout: Option<Duration>,

    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_duration")]
    pub keep_alive_interval: Option<Duration>,

    #[serde(deserialize_with = "deserialize_duration")]
    pub keep_alive_timeout: Duration
}

impl ServiceConfig {
    pub fn axum_config(&self) -> &HyperConfig {
        &self.axum
    }
}
