use axum::Router;
use axum::routing::get;

use crate::SharedState;

pub async fn ping() -> &'static str {
    "pong"
}

pub async fn index() -> &'static str {
    "axum server is running"
}

pub(super) fn create_app(state: SharedState) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/ping", get(ping))
        .with_state(state)
}
