use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Http Service {name} start error")]
    StartFailed {
        name: String,
        #[source]
        source: std::io::Error
    },

    #[error("Rustls failed: {0}")]
    TlsError(#[from] rustls::Error),

    #[error("certificate error: {path}")]
    CertError {
        path: PathBuf,
        #[source]
        source: rustls::pki_types::pem::Error
    },

    #[error("key error: {path}")]
    KeyError {
        path: PathBuf,
        #[source]
        source: rustls::pki_types::pem::Error
    },

    #[error("Unexpected error: {0}")]
    Unexpected(String)
}
