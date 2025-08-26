use std::{net::SocketAddr, sync::Arc, time::Duration};

use axum_server::{tls_rustls::RustlsConfig, Handle};
use hyper_util::rt::TokioTimer;
use rustls::ServerConfig;

use super::app::create_app;
use crate::SharedState;

pub async fn start_https(
    state: SharedState,
    rustls_config: ServerConfig
) -> crate::Result {
    let service_config = &state.config_ref().service_config();
    let ports = &service_config.ports;
    let address = format!("{}:{}", &service_config.address, ports.https);

    let config = &service_config.axum_config();

    log::debug!("🚀 Starting HTTP/2 server on {}", &address);
    let handle = create_handle(state.clone(), config.graceful_timeout);

    let listen_handle = handle.clone();
    tokio::spawn(async move {
        if let Some(addr) = listen_handle.listening().await {
            log::info!("🌐 HTTP/2 server(axum) Listening on {addr}");
        }
    });

    let app = create_app(state.clone());

    let socket_address: SocketAddr = address.parse().unwrap();

    let rustls_config = RustlsConfig::from_config(Arc::new(rustls_config));
    let mut server = axum_server::bind_rustls(socket_address, rustls_config);
    let mut builder = server.http_builder().http2();

    builder
        // .enable_connect_protocol() // on web_socket tunnel proxy
        .auto_date_header(true)
        .timer(TokioTimer::new())
        .keep_alive_timeout(config.keep_alive_timeout)
        .keep_alive_interval(config.keep_alive_interval)
        .max_concurrent_streams(config.max_concurrent_streams)
        .adaptive_window(true)
        // .initial_connection_window_size(1_048_576) // 1 MB //adaptive_window takes control
        // .initial_stream_window_size(512_000)   //adaptive_window takes control
        .max_frame_size(1024 * 16) // 16 KB (RFC default)
        .max_header_list_size(1024 * 16) // 16 KB
        .max_pending_accept_reset_streams(20) // ❗️if non default 20 from h2 crate
        .max_local_error_reset_streams(1024); // ❗️DDOS CVE-2019-9514

    server.handle(handle.clone()).serve(app.into_make_service()).await?;

    log::info!("❎ HTTP/2 (axum) Server shutdown successfully");
    Ok(())
}

fn create_handle(
    state: SharedState,
    graceful_timeout: Option<Duration>
) -> Handle {
    let handle = Handle::new();
    let shutdown_handle = handle.clone();
    tokio::spawn(async move {
        state.shutdown_token().cancelled().await;
        log::warn!("🛑 HTTP/2 Server shutting down...");
        shutdown_handle.graceful_shutdown(graceful_timeout);
    });
    handle
}
