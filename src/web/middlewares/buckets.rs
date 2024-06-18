use axum::{
    body::Body,
    extract::{Path, Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};

use crate::{
    files::queries::buckets::get_bucket,
    uuid::valid_id,
    web::{error::ErrorResponse, server::AppState},
};

pub async fn bucket_middleware(
    State(state): State<AppState>,
    Path(bucket_id): Path<String>,
    mut request: Request,
    next: Next,
) -> Response<Body> {
    let pool = state.db_pool.clone();
    let bid = bucket_id.clone();
    if !valid_id(bid.as_str()) {
        return create_response(
            StatusCode::BAD_REQUEST,
            ErrorResponse {
                status_code: StatusCode::BAD_REQUEST.as_u16(),
                message: "Invalid bucket id".to_string(),
                error: "Bad Request".to_string(),
            },
        );
    }

    let query_res = get_bucket(pool, bid.as_str()).await;
    let Ok(bucket_res) = query_res else {
        return create_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            ErrorResponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                message: "Error getting bucket".to_string(),
                error: "Internal Server Error".to_string(),
            },
        );
    };

    let Some(bucket) = bucket_res else {
        return create_response(
            StatusCode::NOT_FOUND,
            ErrorResponse {
                status_code: StatusCode::NOT_FOUND.as_u16(),
                message: "Bucket not found".to_string(),
                error: "Not Found".to_string(),
            },
        );
    };

    // Forward to the next middleware/handler passing the bucket information
    request.extensions_mut().insert(bucket);
    let response = next.run(request).await;
    response
}

fn create_response(status: StatusCode, body: ErrorResponse) -> Response<Body> {
    Response::builder()
        .status(status)
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap()
}
