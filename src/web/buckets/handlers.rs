use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    Extension,
};

use crate::{
    files::{
        models::{Bucket, NewBucket, UpdateBucket},
        queries::buckets::{create_bucket, delete_bucket, get_bucket, list_buckets, update_bucket},
    },
    web::{response::JsonResponse, server::AppState},
    Error, Result,
};

pub async fn list_buckets_handler(State(state): State<AppState>) -> Result<JsonResponse> {
    let res = list_buckets(&state.db_pool, &state.config.client_id).await;
    let Ok(buckets) = res else {
        return Err("Failed to list buckets".into());
    };

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

pub async fn update_bucket_handler(
    State(state): State<AppState>,
    Extension(bucket): Extension<Bucket>,
    Path(bucket_id): Path<String>,
    payload: Option<Json<UpdateBucket>>,
) -> Result<JsonResponse> {
    let Some(data) = payload else {
        return Err(Error::BadRequest("Invalid request payload".to_string()));
    };
    let updated = update_bucket(&state.db_pool, &bucket_id, &data).await?;

    // Either return the updated bucket or the original one
    match updated {
        true => get_bucket_as_response(&state, &bucket_id).await,
        false => Ok(JsonResponse::new(serde_json::to_string(&bucket).unwrap())),
    }
}

async fn get_bucket_as_response(state: &AppState, id: &str) -> Result<JsonResponse> {
    let res = get_bucket(&state.db_pool, id).await;
    let Ok(bucket_res) = res else {
        return Err("Error getting bucket".into());
    };

    let Some(bucket) = bucket_res else {
        return Err("Error getting bucket this time".into());
    };

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
