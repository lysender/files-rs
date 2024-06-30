use clap::{Parser, Subcommand};
use dotenvy::dotenv;
use serde::Deserialize;
use std::env;

use crate::{util::valid_id, Result};

pub const DATABASE_URL: &str = "DATABASE_URL";
pub const CLIENT_ID: &str = "CLIENT_ID";
pub const JWT_SECRET: &str = "JWT_SECRET";
pub const ADMIN_HASH: &str = "ADMIN_HASH";
pub const GOOGLE_APPLICATION_CREDENTIALS: &str = "GOOGLE_APPLICATION_CREDENTIALS";
pub const GOOGLE_PROJECT_ID: &str = "GOOGLE_PROJECT_ID";
pub const PORT: &str = "PORT";
pub const RUST_LOG: &str = "RUST_LOG";

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub client_id: String,
    pub jwt_secret: String,
    pub cloud_credentials: String,
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
        let database_url = env::var(DATABASE_URL).expect("DATABASE_URL must be set");
        let client_id = env::var(CLIENT_ID).expect("CLIENT_ID must be set");
        let jwt_secret = env::var(JWT_SECRET).expect("JWT_SECRET must be set");
        let cloud_credentials = env::var(GOOGLE_APPLICATION_CREDENTIALS)
            .expect("GOOGLE_APPLICATION_CREDENTIALS must be set");
        let port_str = env::var(PORT).expect("PORT must be set");

        if database_url.len() == 0 {
            return Err("DATABASE_URL is required.".into());
        }
        if client_id.len() == 0 {
            return Err("CLIENT_ID is required.".into());
        }
        if !valid_id(&client_id) {
            return Err("CLIENT_ID is not a valid id.".into());
        }
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
            cloud_credentials,
            server: ServerConfig { port },
            db: DbConfig { url: database_url },
        })
    }
}

/// File Management in the cloud
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Runs the API server
    Server,

    /// Manages clients
    #[command(subcommand)]
    Clients(ClientCommand),

    /// Manages client users
    #[command(subcommand)]
    Users(UserCommand),

    /// Manages client buckets
    #[command(subcommand)]
    Buckets(BucketCommand),

    /// Checks health of the API server
    CheckHealth,

    /// Generates a login credential
    GenerateLogin,
}

#[derive(Subcommand, Debug)]
pub enum ClientCommand {
    List,
    Create,
    Enable,
    Disable,
    Delete,
}

#[derive(Subcommand, Debug)]
pub enum UserCommand {
    List,
    Create,
    Enable,
    Disable,
    Delete,
}

#[derive(Subcommand, Debug)]
pub enum BucketCommand {
    List,
    Create,
    Delete,
}
