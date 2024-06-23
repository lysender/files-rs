use std::process;

use crate::config::Args;
use crate::config::{Commands, Config};
use crate::web::server::run_web_server;
use crate::{auth::admin::generate_admin_hash, Result};

pub async fn run_command(args: Args) -> Result<()> {
    match args.command {
        Commands::Server => run_server().await,
        Commands::CheckHealth => {
            println!("Checking health...");
            Ok(())
        }
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
    todo!()
}

fn generate_login() -> Result<()> {
    generate_admin_hash()
}
