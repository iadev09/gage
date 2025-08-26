use axum::{
    http::{
        uri::{Authority, Scheme},
        StatusCode, Uri
    },
    response::IntoResponse,
    routing::get,
    Router
};
use hyper::header;
use crate::config::Ports;

pub fn create_redirect_only(ports: &Ports) -> Router {
    let https_port = ports.https;

    Router::new().fallback(get(
        move |uri: Uri, headers: axum::http::HeaderMap| async move {
            let response = (|| -> Option<_> {
                let mut parts = uri.into_parts();
                parts.scheme = Some(Scheme::HTTPS);

                if parts.path_and_query.is_none() {
                    parts.path_and_query = Some("/".parse().unwrap());
                }

                let orig_authority = headers
                    .get("host")
                    .and_then(|h| h.to_str().ok())
                    .filter(|s| !s.is_empty())
                    .expect("Invalid host header");

                let authority: Authority =
                    orig_authority.parse().expect("Failed to parse authority");

                let bare_host = authority.host();
                let new_authority = format!("{bare_host}:{https_port}");

                parts.authority = Some(new_authority.parse().ok()?);
                let new_uri = Uri::from_parts(parts).ok()?;

                Some(
                    (
                        StatusCode::MOVED_PERMANENTLY,
                        [(header::LOCATION, new_uri.to_string())]
                    )
                        .into_response()
                )
            })();

            response.unwrap_or_else(|| {
                (StatusCode::BAD_REQUEST, "Invalid URI").into_response()
            })
        }
    ))
}
