use base64::engine::general_purpose::STANDARD;
use base64::Engine;

use crate::ctx::State;

impl State {
    pub(super) fn load_secret() -> Result<[u8; 32], Error> {
        // Fetch and decode the secret from APP_KEY environment variable
        let key = std::env::var("APP_KEY").map_err(|_| Error::MissingEnv("APP_KEY".to_string()))?; // your custom error

        let raw = key.strip_prefix("base64:").unwrap_or(&key);

        let secret: [u8; 32] =
            STANDARD.decode(raw)?.try_into().map_err(|_| Error::AppKeyInvalid)?;

        Ok(secret)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Environment variable not set: {0}")]
    MissingEnv(String),

    #[error("APP_KEY invalid")]
    AppKeyInvalid,

    #[error("Key Error: {0}")]
    KeyDecodeError(#[from] base64::DecodeError)
}
