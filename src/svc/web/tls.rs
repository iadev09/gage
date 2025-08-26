use rustls::ServerConfig;
use serde::Deserialize;

use super::Error;
use crate::svc::web::pem::{load_certificates, load_key};

pub fn create_tls_config(config: &TlsConfig) -> Result<ServerConfig, Error> {
    let key = load_key(&config.key_path)?;
    let certs = load_certificates(&config.cert_path)?;

    let mut server_config = ServerConfig::builder().with_no_client_auth().with_single_cert(certs, key)?;

    // Set ALPN protocols based on config.protocols
    server_config.alpn_protocols = config.protocols.iter().map(|p| p.alpn_protocol().to_vec()).collect();

    Ok(server_config)
}

pub fn install_default_provider() {
    rustls::crypto::aws_lc_rs::default_provider().install_default().unwrap();
}

#[derive(Deserialize)]
pub struct TlsConfig {
    cert_path: String,
    key_path: String,
    protocols: Vec<Protocol>
}

#[derive(Debug, Deserialize)]
enum Protocol {
    Http1,
    Http2,
    Http3
}

impl Protocol {
    fn alpn_protocol(&self) -> &[u8] {
        match self {
            Protocol::Http1 => b"http/1.1",
            Protocol::Http2 => b"h2",
            Protocol::Http3 => b"http/3"
        }
    }
}
