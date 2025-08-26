use actix_web::{http::KeepAlive, web, App, HttpServer};
use rustls::ServerConfig;

use crate::{
    ctx::SharedState,
    svc::web::{actix::app, error::Error}
};

pub async fn start_https(
    state: SharedState,
    tls_config: ServerConfig
) -> crate::Result {
    let service_config = state.config_ref().service_config();
    let config = &service_config.actix_config();
    let address = format!(
        "{}:{}",
        service_config.address.as_str(),
        service_config.ports.https
    );
    log::info!("🚀 Starting HTTP/2 (actix) server on {}", &address);
    let app_state = state.clone();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .configure(app::configure)
    })
    .shutdown_signal(state.shutdown_token().cancelled_owned())
    .server_hostname(service_config.hostname.as_str())
    .max_connections(config.max_connections)
    .max_connection_rate(config.max_connection_rate)
    .shutdown_timeout(config.shutdown_timeout)
    .client_disconnect_timeout(config.client_disconnect_timeout)
    .keep_alive(KeepAlive::from(config.keep_alive_interval))
    .client_request_timeout(config.client_request_timeout)
    .tls_handshake_timeout(config.tls_handshake_timeout)
    // .on_connect(|x, extra| log::info!("Client connected: {:?} {:?}", x, extra))
    .bind_rustls_0_23(address, tls_config)?
    .run()
    .await
    .map_err(|e| Error::StartFailed { name: "Actix".to_string(), source: e })?;

    log::info!("❎ Server shutdown gracefully. Exiting.");
    Ok(())
}
