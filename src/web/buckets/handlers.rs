use axum::{
    body::Body,
    extract::{Json, Path, State},
    http::StatusCode,
    response::Response,
    Extension,
};

use crate::{
    files::{
        models::{Bucket, NewBucket},
        queries::buckets::{create_bucket, list_buckets},
    },
    web::{
        response::{create_error_response, create_response, create_success_response},
        server::AppState,
    },
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

    create_success_response(serde_json::to_string(&buckets).unwrap())
}

pub async fn create_bucket_handler(
    State(state): State<AppState>,
    Json(payload): Json<NewBucket>,
) -> Response<Body> {
    let pool = state.db_pool.clone();
    let config = state.config.clone();
    let client_id = config.client_id.clone();

    println!("Creating bucket: {:?}", payload);

    let bucket_res = create_bucket(pool, client_id.as_str(), payload).await;
    let Ok(bucket) = bucket_res else {
        return create_error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to list buckets".to_string(),
            "Internal Server Error".to_string(),
        );
    };

    create_response(StatusCode::CREATED, serde_json::to_string(&bucket).unwrap())
}

pub async fn get_bucket_handler(Extension(bucket): Extension<Bucket>) -> Response<Body> {
    // Extract bucket from the middleware extension
    return create_success_response(serde_json::to_string(&bucket).unwrap());
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
