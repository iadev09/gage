use crate::ctx::State;

pub(crate) struct Info {
    app: String,
    user: String,
    hostname: String,
    work_dir: String
}

#[allow(unused)]
impl Info {
    pub fn new(
        user: String,
        hostname: String,
        work_dir: String
    ) -> Self {
        let app = env!("CARGO_PKG_NAME").to_string();
        Self { app, user, hostname, work_dir }
    }

    pub fn from_env() -> Result<Self, Error> {
        let user = std::env::var("USER").map_err(Error::UserEnvVar)?;
        let work_dir = std::env::current_dir().map_err(Error::CurrentDir)?.to_string_lossy().to_string();
        let hostname = hostname::get()
            .map_err(Error::Hostname)?
            .to_string_lossy()
            .split('.')
            .next()
            .unwrap_or_default()
            .to_string();
        Ok(Self::new(user, hostname, work_dir))
    }
}

#[allow(unused)]
impl State {
    pub fn get_current_user(&self) -> &str {
        self.info.user.as_ref()
    }

    pub fn get_working_dir(&self) -> &str {
        self.info.work_dir.as_ref()
    }

    pub fn get_hostname(&self) -> &str {
        self.info.hostname.as_ref()
    }

    pub fn my_name(&self) -> &str {
        self.info.app.as_ref()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Unable to get environment variable USER: {0}")]
    UserEnvVar(#[source] std::env::VarError),

    #[error("Unable to get hostname: {0}")]
    Hostname(#[source] std::io::Error),

    #[error("Unable to get current directory: {0}")]
    CurrentDir(#[source] std::io::Error)
}
