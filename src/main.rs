use clap::Parser;
use config::Args;
use run::run_command;
use std::process;

mod auth;
mod config;
mod db;
mod error;
mod files;
mod health;
mod run;
mod schema;
mod util;
mod validators;
mod web;

// Re-export error types for convenience
pub use self::error::{Error, Result};

#[tokio::main]
async fn main() {
    // Set the RUST_LOG, if it hasn't been explicitly defined
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "files_rs=info")
    }

    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let args = Args::parse();

    if let Err(e) = run_command(args).await {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
