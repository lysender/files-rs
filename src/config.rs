use clap::{Parser, Subcommand};
use serde::Deserialize;
use std::path::Path;
use std::{fs, path::PathBuf};

use crate::Result;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub client_id: String,
    pub server: ServerConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
}

impl Config {
    pub fn build(filename: &Path) -> Result<Self> {
        let toml_string = match fs::read_to_string(filename) {
            Ok(str) => str,
            Err(_) => {
                return Err("Unable to read config file.".into());
            }
        };

        let config: Config = match toml::from_str(toml_string.as_str()) {
            Ok(value) => value,
            Err(_) => {
                return Err("Unable to parse config file.".into());
            }
        };

        if config.client_id.len() == 0 {
            return Err("Auth client_id is required.".into());
        }
        if config.server.port == 0 {
            return Err("Server port is required.".into());
        }

        Ok(config)
    }
}

/// CLI tool to create issues into a project
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// TOML configuration file
    #[arg(short, long, value_name = "FILE.toml")]
    pub config: PathBuf,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Runs the API server
    Server,
}
