use std::process;

use crate::clients::run_client_command;
use crate::config::{BucketCommand, CliArgs};
use crate::config::{Commands, Config};
use crate::db::create_db_pool;
use crate::health::check_readiness;
use crate::users::run_user_command;
use crate::web::server::run_web_server;
use crate::{auth::generate_admin_hash, Result};

pub async fn run_command(args: CliArgs) -> Result<()> {
    let config = Config::build().unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });
    match args.command {
        Commands::Server => run_server(config).await,
        Commands::Clients(cmd) => run_client_command(cmd).await,
        Commands::Buckets(cmd) => manage_buckets(config, cmd).await,
        Commands::Users(cmd) => run_user_command(cmd).await,
        Commands::CheckHealth => check_health().await,
        Commands::GenerateLogin => generate_login(),
    }
}

pub async fn run_server(config: Config) -> Result<()> {
    run_web_server(config).await
}

pub async fn manage_buckets(config: Config, cmd: BucketCommand) -> Result<()> {
    println!("Manage buckets");
    println!("{:?}", cmd);
    Ok(())
}

pub async fn check_health() -> Result<()> {
    let pool = create_db_pool();
    let health = check_readiness(&pool).await?;

    // Print the health status
    println!("Status: {}", health.status);
    println!("Message: {}", health.message);
    println!("Checks:");
    println!("  Auth: {}", health.checks.auth);
    println!("  Cloud Storage: {}", health.checks.cloud_storage);
    println!("  Database: {}", health.checks.database);
    println!("  Secrets: {}", health.checks.secrets);

    Ok(())
}

fn generate_login() -> Result<()> {
    generate_admin_hash()
}
