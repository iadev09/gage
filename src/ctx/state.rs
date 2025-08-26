use std::{future::Future, sync::Arc};

use clap::Parser;
use tokio_util::sync::CancellationToken;

use crate::ctx::{config::Config, error::Error, info::Info, options::Options};

pub type SharedState = Arc<State>;

#[allow(unused)]
pub struct State {
    pub(crate) info: Info,
    config: Arc<Config>,
    shutdown_token: CancellationToken,
    secret: [u8; 32]
}

#[allow(unused)]
impl State {
    pub fn shared() -> Result<Arc<Self>, Error> {
        let options = Options::parse();
        let config = Config::from_file(&options.config_path)?;
        Ok(Arc::new(Self {
            secret: Self::load_secret()?,
            config: Arc::new(config),
            info: Info::from_env()?,
            shutdown_token: CancellationToken::new()
        }))
    }

    pub fn get_secret(&self) -> &[u8; 32] {
        &self.secret
    }

    pub fn config(&self) -> Arc<Config> {
        self.config.clone()
    }

    pub fn config_ref(&self) -> &Config {
        &self.config
    }
}

#[allow(unused)]
impl State {
    pub fn is_shutting_down(&self) -> bool {
        self.shutdown_token.is_cancelled()
    }

    pub fn initiate_shutdown(&self) {
        self.shutdown_token.cancel();
        log::warn!("💥 Shutdown initiated. Graceful shutdown in progress...");
    }

    pub fn on_shutdown(&self) -> impl Future<Output = ()> + '_ {
        self.shutdown_token.cancelled()
    }

    pub fn shutdown_token(&self) -> CancellationToken {
        self.shutdown_token.clone()
    }
}
