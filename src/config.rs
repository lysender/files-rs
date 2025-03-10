use clap::{Parser, Subcommand};
use dotenvy::dotenv;
use serde::Deserialize;
use std::{env, path::PathBuf};

use crate::Result;

pub const DATABASE_URL: &str = "DATABASE_URL";
pub const UPLOAD_DIR: &str = "UPLOAD_DIR";
pub const JWT_SECRET: &str = "JWT_SECRET";
pub const GOOGLE_APPLICATION_CREDENTIALS: &str = "GOOGLE_APPLICATION_CREDENTIALS";
pub const GOOGLE_PROJECT_ID: &str = "GOOGLE_PROJECT_ID";
pub const PORT: &str = "PORT";

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub jwt_secret: String,
    pub cloud_credentials: String,
    pub upload_dir: PathBuf,
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
        let jwt_secret = env::var(JWT_SECRET).expect("JWT_SECRET must be set");
        let cloud_credentials = env::var(GOOGLE_APPLICATION_CREDENTIALS)
            .expect("GOOGLE_APPLICATION_CREDENTIALS must be set");
        let mut upload_dir = PathBuf::from(env::var(UPLOAD_DIR).expect("UPLOAD_DIR must be set"));
        let port_str = env::var(PORT).expect("PORT must be set");

        if database_url.len() == 0 {
            return Err("DATABASE_URL is required.".into());
        }
        if jwt_secret.len() == 0 {
            return Err("JWT_SECRET is required.".into());
        }

        // Validate upload dir, it should exist first, then we will create tmp dir inside
        if !upload_dir.exists() {
            return Err("UPLOAD_DIR does not exist.".into());
        }
        upload_dir = upload_dir.join("tmp");

        std::fs::create_dir_all(&upload_dir).expect("Unable to create upload tmp dir");

        let Ok(port) = port_str.parse::<u16>() else {
            return Err("PORT must be a valid number.".into());
        };

        if port == 0 {
            return Err("PORT is required.".into());
        }

        Ok(Self {
            jwt_secret,
            cloud_credentials,
            upload_dir,
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
}

#[derive(Subcommand, Debug)]
pub enum ClientCommand {
    List,
    Create { name: String },
    Enable { id: String },
    Disable { id: String },
    Delete { id: String },
    SetDefaultBucket { id: String, bucket_id: String },
    UnsetDefaultBucket { id: String },
}

#[derive(Subcommand, Debug)]
pub enum UserCommand {
    List {
        client_id: String,
    },
    Create {
        client_id: String,
        username: String,
        roles: String,
    },
    Password {
        id: String,
    },
    Enable {
        id: String,
    },
    Disable {
        id: String,
    },
    Delete {
        id: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum BucketCommand {
    List {
        client_id: String,
    },
    Create {
        client_id: String,
        name: String,
        images_only: String,
    },
    Delete {
        id: String,
    },
}
