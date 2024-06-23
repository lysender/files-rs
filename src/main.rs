use clap::Parser;
use config::Args;
use config::Commands;
use config::Config;
use std::process;

use crate::web::server::run;

mod auth;
mod config;
mod db;
mod error;
mod files;
mod schema;
mod util;
mod validators;
mod web;

// Re-export error types for convenience
pub use self::error::{Error, Result};

#[tokio::main]
async fn main() {
    // Set the RUST_LOG, if it hasn't been explicitly defined
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "files_rs=info")
    }

    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let args = Args::parse();
    let config = Config::build().unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    if let Err(e) = run_command(args, config).await {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

async fn run_command(args: Args, config: Config) -> Result<()> {
    match args.command {
        Commands::Server => match run(config).await {
            Ok(_) => Ok(()),
            Err(err) => {
                eprintln!("{err}");
                process::exit(1);
            }
        },
        Commands::CheckHealth => {
            println!("Checking health...");
            Ok(())
        }
        Commands::GenerateLogin => {
            println!("Generating login...");
            Ok(())
        }
    }
}
