use clap::{Parser, Subcommand};
use dotenvy::dotenv;
use serde::Deserialize;
use std::env;

use crate::{
    util::{base64_decode, valid_id},
    Result,
};

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub client_id: String,
    pub admin_hash: String,
    pub jwt_secret: String,
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
        let admin_hash_base64 = env::var("ADMIN_HASH").expect("ADMIN_HASH must be set");
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
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
        if admin_hash_base64.len() == 0 {
            return Err("ADMIN_HASH is required.".into());
        }
        let Ok(admin_hash) = base64_decode(&admin_hash_base64) else {
            return Err("ADMIN_HASH must be a valid base64 string.".into());
        };

        if jwt_secret.len() == 0 {
            return Err("JWT_SECRET is required.".into());
        }

        let Ok(port) = port_str.parse::<u16>() else {
            return Err("PORT must be a valid number.".into());
        };

        if port == 0 {
            return Err("PORT is required.".into());
        }

        Ok(Self {
            client_id,
            jwt_secret,
            admin_hash,
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

    /// Checks health of the API server
    CheckHealth,

    /// Generates a login credential
    GenerateLogin,
}
