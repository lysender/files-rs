use crate::web::error::ErrorResponse;
use crate::web::server::AppState;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

pub async fn not_found_handler(_state: State<AppState>) -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(ErrorResponse {
            status_code: StatusCode::NOT_FOUND.as_u16(),
            message: "Resource not found".to_string(),
            error: "Not Found".to_string(),
        }),
    )
}
