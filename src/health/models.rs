use serde::Serialize;

#[derive(Serialize)]
pub struct LiveStatus {
    pub status: String,
}

#[derive(Serialize)]
pub struct HealthStatus {
    pub status: String,
    pub message: String,
    pub checks: HealthChecks,
}

#[derive(Serialize)]
pub struct HealthChecks {
    pub auth: String,
    pub cloud_storage: String,
    pub database: String,
    pub secrets: String,
}
