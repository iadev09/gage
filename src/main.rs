#![cfg_attr(all(feature = "ntex", feature = "actix"), allow(unused))]
#[cfg(all(feature = "ntex", feature = "actix"))]
compile_error!("Enable only one runtime: feature 'ntex' OR feature 'actix'.");

use std::process;

use dotenvy::dotenv;
use futures;
use gage::{
    logging,
    svc::{shutdown, tasks::sample, web},
    Result, State
};
use heck::ToPascalCase;
use web::tls;

#[cfg_attr(feature = "ntex", ntex::main)]
#[cfg_attr(all(not(feature = "ntex"), feature = "actix"), actix_web::main)]
#[cfg_attr(all(not(feature = "ntex"), not(feature = "actix")), tokio::main)]
async fn main() -> Result {
    dotenv().ok();

    logging::init_log().await?;

    let state = State::shared()?;

    let listener = {
        #[cfg(feature = "ntex")]
        {
            ntex::rt::spawn(shutdown::listen(state.clone()))
        }
        #[cfg(all(not(feature = "ntex"), feature = "actix"))]
        {
            actix_web::rt::spawn(shutdown::listen(state.clone()))
        }
        #[cfg(all(not(feature = "ntex"), not(feature = "actix")))]
        {
            tokio::spawn(shutdown::listen(state.clone()))
        }
    };

    tls::install_default_provider();
    let service_config = state.config_ref().service_config();
    let tls_config = tls::create_tls_config(&service_config.tls)?;

    let worker = sample::start_worker(state.clone());

    let http = web::start_http(state.clone());
    let https = web::start_https(state.clone(), tls_config);

    if let Err(err) = futures::try_join!(http, https, worker) {
        log::error!("❌ Service start failed: {err}");
        process::exit(1);
    }

    listener.abort();
    log::info!("🏁 {} exited successfully 🎉", state.my_name().to_pascal_case());
    Ok(())
}
