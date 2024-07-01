use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    Extension,
};

use crate::{
    buckets::{create_bucket, delete_bucket, list_buckets, Bucket, NewBucket},
    web::{response::JsonResponse, server::AppState},
    Error, Result,
};

pub async fn list_buckets_handler(State(state): State<AppState>) -> Result<JsonResponse> {
    let buckets = list_buckets(&state.db_pool, &state.config.client_id).await?;
    Ok(JsonResponse::new(serde_json::to_string(&buckets).unwrap()))
}

pub async fn create_bucket_handler(
    State(state): State<AppState>,
    payload: Option<Json<NewBucket>>,
) -> Result<JsonResponse> {
    let Some(data) = payload else {
        return Err(Error::BadRequest("Invalid request payload".to_string()));
    };
    let bucket = create_bucket(&state.db_pool, &state.config.client_id, &data).await?;
    Ok(JsonResponse::with_status(
        StatusCode::CREATED,
        serde_json::to_string(&bucket).unwrap(),
    ))
}

pub async fn get_bucket_handler(Extension(bucket): Extension<Bucket>) -> Result<JsonResponse> {
    // Extract bucket from the middleware extension
    Ok(JsonResponse::new(serde_json::to_string(&bucket).unwrap()))
}

pub async fn delete_bucket_handler(
    State(state): State<AppState>,
    Path(bucket_id): Path<String>,
) -> Result<JsonResponse> {
    let _ = delete_bucket(&state.db_pool, &bucket_id).await?;
    Ok(JsonResponse::with_status(
        StatusCode::NO_CONTENT,
        "".to_string(),
    ))
}
