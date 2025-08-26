
#[derive(thiserror::Error, Debug)]
pub enum Error {

    #[error("Internal error: {0}")]
    Internal(String)
}

impl From<&str> for Error {
    fn from(error: &str) -> Self {
        Error::Internal(error.to_string())
    }
}

impl From<String> for Error {
    fn from(error: String) -> Self {
        Error::Internal(error)
    }
}
