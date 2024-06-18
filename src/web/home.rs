use axum::{extract::State, response::IntoResponse, Json};
use serde::Serialize;

use super::server::AppState;

#[derive(Serialize)]
pub struct AppMeta {
    pub name: String,
    pub version: String,
}

pub async fn home_handler(State(_state): State<AppState>) -> impl IntoResponse {
    Json(AppMeta {
        name: "files-rs".to_string(),
        version: "0.1.0".to_string(),
    })
}
