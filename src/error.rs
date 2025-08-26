use crate::ctx::logging::Error as LoggingError;
use crate::ctx::CtxError;
use crate::svc;

pub type Result = std::result::Result<(), Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Ctx Failed: {0}")]
    Ctx(#[from] CtxError),

    #[error("Log error: {0}")]
    Logging(#[from] LoggingError),

    #[error("Http error: {0}")]
    HttpError(#[from] svc::web::Error),

    #[error("Io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Unexpected error: {0}")]
    Unexpected(String),

}

impl From<&str> for Error {
    fn from(error: &str) -> Self {
        Error::Unexpected(error.to_string())
    }
}

impl From<String> for Error {
    fn from(error: String) -> Self {
        Error::Unexpected(error)
    }
}
