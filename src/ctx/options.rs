use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "gage", author, version, about = "oAuth2 token exchange service")]
pub(super) struct Options {
    #[arg(default_value = "Gage.toml", help = "config file path")]
    pub config_path: PathBuf
}
