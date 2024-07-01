use clap::Parser;
use config::{CliArgs, RUST_LOG};
use run::run_command;
use std::process;

mod auth;
mod buckets;
mod clients;
mod config;
mod db;
mod dirs;
mod error;
mod files;
mod health;
mod roles;
mod run;
mod schema;
mod storage;
mod users;
mod util;
mod validators;
mod web;

// Re-export error types for convenience
pub use self::error::{Error, Result};

#[tokio::main]
async fn main() {
    // Set the RUST_LOG, if it hasn't been explicitly defined
    if std::env::var(RUST_LOG).is_err() {
        std::env::set_var(RUST_LOG, "files_rs=info")
    }

    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let args = CliArgs::parse();

    if let Err(e) = run_command(args).await {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
