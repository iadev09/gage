#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("State error: {0}")]
    ApiKey(#[from] crate::ctx::vars::Error),

    #[error("Info error: {0}")]
    Info(#[from] crate::ctx::info::Error),

    #[error("Config error: {0}")]
    Config(#[from] crate::ctx::config::Error)
}
