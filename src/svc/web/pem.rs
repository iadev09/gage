use std::path::Path;

use rustls::pki_types::pem::PemObject;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};

use crate::svc::web::Error;

pub fn load_certificates(path: impl AsRef<Path>) -> Result<Vec<CertificateDer<'static>>, Error> {
    log::debug!("Loading certificates from {}", path.as_ref().display());
    let path_ref = path.as_ref();
    let certs = CertificateDer::pem_file_iter(path_ref)
        .map_err(|e| Error::CertError { path: path_ref.to_path_buf(), source: e })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| Error::CertError { path: path_ref.to_path_buf(), source: e })?;
    Ok(certs)
}

pub fn load_key(key: impl AsRef<Path>) -> Result<PrivateKeyDer<'static>, Error> {
    log::debug!("Loading private key from {}", key.as_ref().display());
    let key_ref = key.as_ref();
    let key = PrivateKeyDer::from_pem_file(key_ref)
        .map_err(|e| Error::KeyError { path: key_ref.to_path_buf(), source: e })?;
    Ok(key)
}
