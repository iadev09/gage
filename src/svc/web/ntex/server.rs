use ntex::{
    http::StatusCode,
    web::{self, guard, middleware, App, Error, HttpResponse}
};
use ntex_files as fs;
use ntex_session::CookieSession;
use rustls::ServerConfig;

use super::app;
use crate::SharedState;

/// 404 handler
async fn p404() -> Result<fs::NamedFile, Error> {
    Ok(fs::NamedFile::open("public/404.html")?
        .set_status_code(StatusCode::NOT_FOUND))
}

pub async fn start_https(
    state: SharedState,
    tls_config: ServerConfig
) -> crate::Result {
    let service_config = state.config_ref().service_config();
    let config = &service_config.ntex_config();
    let address = format!(
        "{}:{}",
        service_config.address.as_str(),
        service_config.ports.https
    );
    let app_state = state.clone();

    log::info!("🚀 Starting HTTP/2 (ntex) server on {}", &address);

    web::server(move || {
        App::new()
            .state(app_state.clone())
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            // enable logger
            .wrap(middleware::Logger::default())
            .configure(app::configure)
            .default_service(
                // 404 for GET request
                web::resource("").route(web::get().to(p404)).route(
                    web::route()
                        .guard(guard::Not(guard::Get()))
                        .to(|| async { HttpResponse::MethodNotAllowed() })
                )
            )
    })
    .server_hostname(service_config.hostname.as_str())
    .maxconn(config.max_conn)
    .maxconnrate(config.max_conn_rate)
    .shutdown_timeout(config.shutdown_timeout)
    .disconnect_timeout(config.disconnect_timeout)
    .client_timeout(config.client_timeout)
    .keep_alive(config.keep_alive_interval)
    .ssl_handshake_timeout(config.ssl_handshake_timeout)
    .headers_read_rate(
        config.headers_read_rate.timeout.into(),
        config.headers_read_rate.max_timeout.into(),
        config.headers_read_rate.rate
    )
    .bind_rustls(address, tls_config)?
    .run()
    .await
    .map_err(|e| crate::svc::web::Error::StartFailed {
        name: "Ntex".to_string(),
        source: e
    })?;

    log::info!("❎ Server shutdown gracefully. Exiting.");
    Ok(())
}
