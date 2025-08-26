pub mod tls;

#[cfg(feature = "actix")]
pub(crate) mod actix;

#[cfg(feature = "ntex")]
pub(crate) mod ntex;

#[cfg(feature = "axum")]
pub(crate) mod axum;

mod error;
mod http;
mod pem;
mod redirect;

#[cfg(feature = "actix")]
pub use actix::start_https;
#[cfg(feature = "axum")]
pub use axum::start_https;
pub use error::Error;
pub use http::start_http;
#[cfg(feature = "ntex")]
pub use ntex::start_https;
