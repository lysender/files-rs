use deadpool_diesel::sqlite::Pool;
use dotenvy::dotenv;
use std::env;
use tracing::error;

use crate::{
    buckets::test_read_bucket,
    config::{GOOGLE_PROJECT_ID, JWT_SECRET},
    storage::test_list_buckets,
    Result,
};

use super::{HealthChecks, HealthStatus, LiveStatus};

pub async fn check_liveness() -> Result<LiveStatus> {
    // Nothing much to check, if it hits this function, it's alive
    Ok(LiveStatus {
        status: "UP".to_string(),
    })
}

pub async fn check_readiness(db_pool: &Pool) -> Result<HealthStatus> {
    let checks = perform_checks(db_pool).await?;
    let mut status = "DOWN".to_string();
    let mut message = "One or more health checks are failing".to_string();

    if checks.is_healthy() {
        status = "UP".to_string();
        message = "All health checks are passing".to_string();
    }

    Ok(HealthStatus {
        status,
        message,
        checks,
    })
}

async fn perform_checks(db_pool: &Pool) -> Result<HealthChecks> {
    dotenv().ok();

    let mut checks = HealthChecks::new();

    checks.cloud_storage = check_cloud_storage().await?;
    checks.database = check_database(db_pool).await?;
    checks.secrets = check_secrets().await?;

    Ok(checks)
}

async fn check_cloud_storage() -> Result<String> {
    let Ok(project_id) = env::var(GOOGLE_PROJECT_ID) else {
        error!("GOOGLE_PROJECT_ID is not set");
        return Ok("DOWN".to_string());
    };

    match test_list_buckets(&project_id).await {
        Ok(_) => Ok("UP".to_string()),
        Err(e) => {
            let msg = format!("{}", e);
            error!(msg);
            Ok("DOWN".to_string())
        }
    }
}

async fn check_database(db_pool: &Pool) -> Result<String> {
    match test_read_bucket(db_pool).await {
        Ok(_) => Ok("UP".to_string()),
        Err(e) => {
            let msg = format!("{}", e);
            error!(msg);
            Ok("DOWN".to_string())
        }
    }
}

async fn check_secrets() -> Result<String> {
    match env::var(JWT_SECRET) {
        Ok(_) => Ok("UP".to_string()),
        Err(e) => {
            let msg = format!("{}", e);
            error!(msg);
            Ok("DOWN".to_string())
        }
    }
}
