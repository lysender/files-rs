use axum::{body::Body, http::StatusCode, response::Response};

use crate::web::error::ErrorResponse;

pub fn create_response(status: StatusCode, body: String) -> Response<Body> {
    Response::builder()
        .status(status)
        .header("Content-Type", "application/json")
        .body(Body::from(body))
        .unwrap()
}

pub fn create_success_response(body: String) -> Response<Body> {
    return create_response(StatusCode::OK, body);
}

pub fn create_error_response(status: StatusCode, message: String, error: String) -> Response<Body> {
    let body = ErrorResponse {
        status_code: status.as_u16(),
        message,
        error,
    };

    return create_response(status, serde_json::to_string(&body).unwrap());
}
