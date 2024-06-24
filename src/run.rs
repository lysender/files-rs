use std::process;

use crate::config::Args;
use crate::config::{Commands, Config};
use crate::web::server::run_web_server;
use crate::{auth::generate_admin_hash, Result};

pub async fn run_command(args: Args) -> Result<()> {
    match args.command {
        Commands::Server => run_server().await,
        Commands::CheckHealth => check_health().await,
        Commands::GenerateLogin => generate_login(),
    }
}

pub async fn run_server() -> Result<()> {
    let config = Config::build().unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });
    run_web_server(config).await
}

pub async fn check_health() -> Result<()> {
    println!("Checking health...");
    println!("Passed for now!");
    Ok(())
}

fn generate_login() -> Result<()> {
    generate_admin_hash()
}
