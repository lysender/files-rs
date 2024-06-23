use axum::extract::State;

use super::{response::JsonResponse, server::AppState};

use crate::{
    health::{HealthChecks, HealthStatus, LiveStatus},
    Result,
};

pub async fn health_live_handler(State(state): State<AppState>) -> Result<JsonResponse> {
    // Just a dummy health check for now
    let health = LiveStatus {
        status: "UP".to_string(),
    };

    Ok(JsonResponse::new(serde_json::to_string(&health).unwrap()))
}

pub async fn health_ready_handler(State(state): State<AppState>) -> Result<JsonResponse> {
    // Just a dummy health check for now
    let health = HealthStatus {
        status: "UP".to_string(),
        message: "Service is up and running".to_string(),
        checks: HealthChecks {
            auth: "UP".to_string(),
            cloud_storage: "UP".to_string(),
            database: "UP".to_string(),
            secrets: "UP".to_string(),
        },
    };

    Ok(JsonResponse::new(serde_json::to_string(&health).unwrap()))
}
