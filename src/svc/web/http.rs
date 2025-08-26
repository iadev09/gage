use tokio::net::TcpListener;

use super::redirect::create_redirect_only;
use crate::SharedState;

pub async fn start_http(state: SharedState) -> crate::Result {
    let service_config = &state.config_ref().service_config();
    let ports = &service_config.ports;

    let app = create_redirect_only(ports);

    let address = format!("{}:{}", &service_config.address, ports.http);

    log::info!("🚀 Starting HTTP (redirect) server on {}", &address);

    let listener = TcpListener::bind(address.as_str()).await.map_err(|e| {
        log::error!("Failed to bind HTTP/1 TCP listener on {}: {:?}", &address, e);
        crate::Error::from(e)
    })?;

    // .enable_connect_protocol() // on web_socket tunnel proxy
    let shutdown_token = state.shutdown_token();
    axum::serve(listener, app)
        .with_graceful_shutdown(async move {
            shutdown_token.cancelled().await;
            log::warn!("🔸 HTTP (redirect) Server shutting down...");
        })
        .await?;

    log::info!("❎ HTTP (redirect) Server shutdown successfully");
    Ok(())
}
