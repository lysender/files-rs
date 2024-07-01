use crate::buckets::run_bucket_command;
use crate::clients::run_client_command;
use crate::config::CliArgs;
use crate::config::Commands;
use crate::db::create_db_pool;
use crate::health::check_readiness;
use crate::users::run_user_command;
use crate::web::server::run_web_server;
use crate::Result;

pub async fn run_command(args: CliArgs) -> Result<()> {
    match args.command {
        Commands::Server => run_web_server().await,
        Commands::Clients(cmd) => run_client_command(cmd).await,
        Commands::Buckets(cmd) => run_bucket_command(cmd).await,
        Commands::Users(cmd) => run_user_command(cmd).await,
        Commands::CheckHealth => check_health().await,
    }
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
