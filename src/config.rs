use clap::{Parser, Subcommand};
use dotenvy::dotenv;
use serde::Deserialize;
use std::env;

use crate::{util::valid_id, Result};

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub client_id: String,
    pub server: ServerConfig,
    pub db: DbConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DbConfig {
    pub url: String,
}

impl Config {
    pub fn build() -> Result<Self> {
        // Load configuration from environment variables
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let client_id = env::var("CLIENT_ID").expect("CLIENT_ID must be set");
        let port_str = env::var("PORT").expect("PORT must be set");

        if database_url.len() == 0 {
            return Err("DATABASE_URL is required.".into());
        }
        if client_id.len() == 0 {
            return Err("CLIENT_ID is required.".into());
        }
        if !valid_id(&client_id) {
            return Err("CLIENT_ID is not a valid id.".into());
        }

        let Ok(port) = port_str.parse::<u16>() else {
            return Err("PORT must be a valid number.".into());
        };

        if port == 0 {
            return Err("PORT is required.".into());
        }

        Ok(Self {
            client_id,
            server: ServerConfig { port },
            db: DbConfig { url: database_url },
        })
    }
}

/// CLI tool to create issues into a project
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Runs the API server
    Server,
}
