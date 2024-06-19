use axum::{
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};

use crate::{
    files::{models::Bucket, queries::buckets::list_buckets},
    web::{error::create_error_response, server::AppState},
};

pub async fn list_buckets_handler(State(state): State<AppState>) -> Response<Body> {
    let pool = state.db_pool.clone();
    let config = state.config.clone();
    let client_id = config.client_id.clone();

    let buckets_res = list_buckets(pool, client_id.as_str()).await;
    let Ok(buckets) = buckets_res else {
        return create_error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to list buckets".to_string(),
            "Internal Server Error".to_string(),
        );
    };

    let res = Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(serde_json::to_string(&buckets).unwrap()))
        .unwrap();
    res
}

pub async fn create_bucket_handler(State(_state): State<AppState>) -> Response<Body> {
    let r = Response::builder().status((StatusCode::OK).as_u16());
    let body = "Home page".to_string();
    r.body(Body::from(body)).unwrap()
}

pub async fn get_bucket_handler(Extension(bucket): Extension<Bucket>) -> impl IntoResponse {
    // Extract bucket from the middleware extension
    Json(bucket)
}

pub async fn update_bucket_handler(State(_state): State<AppState>) -> Response<Body> {
    let r = Response::builder().status((StatusCode::OK).as_u16());
    let body = "update bucket".to_string();
    r.body(Body::from(body)).unwrap()
}

pub async fn delete_bucket_handler(State(_state): State<AppState>) -> Response<Body> {
    let r = Response::builder().status((StatusCode::OK).as_u16());
    let body = "delete bucket".to_string();
    r.body(Body::from(body)).unwrap()
}
