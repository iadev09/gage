use std::time::Duration;

use serde::Deserialize;
use serde_with::serde_as;

use crate::{config::ServiceConfig, ctx::utils::deserialize_duration};

#[serde_as]
#[derive(Deserialize)]
pub struct ActixConfig {
    pub max_connections: usize,

    pub max_connection_rate: usize,

    pub shutdown_timeout: u64,

    #[serde(deserialize_with = "deserialize_duration")]
    pub client_disconnect_timeout: Duration,

    #[serde(deserialize_with = "deserialize_duration")]
    pub client_request_timeout: Duration,

    #[serde(deserialize_with = "deserialize_duration")]
    pub tls_handshake_timeout: Duration,

    #[serde(deserialize_with = "deserialize_duration")]
    pub keep_alive_interval: Duration
}

impl ServiceConfig {
    pub fn actix_config(&self) -> &ActixConfig {
        &self.actix
    }
}
