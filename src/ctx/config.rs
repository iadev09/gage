use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub(crate) service: ServiceConfig
}

impl Config {
    pub fn service_config(&self) -> &ServiceConfig {
        &self.service
    }
}

#[derive(Deserialize, Clone, Copy)]
pub(crate) struct Ports {
    pub(crate) http: u16,
    pub(crate) https: u16
}

#[derive(Deserialize)]
pub struct ServiceConfig {
    pub address: String,

    pub hostname: String,

    #[allow(unused)]
    pub(crate) ports: Ports,

    #[cfg(feature = "ntex")]
    pub(crate) ntex: crate::svc::web::ntex::config::NtexConfig,

    #[cfg(feature = "actix")]
    pub(crate) actix: crate::svc::web::actix::config::ActixConfig,
    #[cfg(feature = "axum")]
    pub(crate) axum: crate::svc::web::axum::config::HyperConfig,

    pub tls: crate::svc::web::tls::TlsConfig
}

impl Config {
    pub fn from_file(path: &PathBuf) -> Result<Config, Error> {
        let content = std::fs::read_to_string(&path)
            .map_err(|e| Error::File { path: path.clone(), source: e })?;
        let config: Config = toml::from_str(&content)
            .map_err(|e| Error::Parser { path: path.clone(), source: e })?;
        Ok(config)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("config file error: {path}")]
    File {
        path: PathBuf,
        #[source]
        source: std::io::Error
    },

    #[error("config parse error in {path} (expected TOML)")]
    Parser {
        path: PathBuf,
        #[source]
        source: toml::de::Error
    }
}
