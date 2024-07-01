use axum::{extract::State, Extension};

use crate::{
    auth::ActorDto,
    buckets::{list_buckets, Bucket},
    web::{response::JsonResponse, server::AppState},
    Result,
};

pub async fn list_buckets_handler(
    State(state): State<AppState>,
    Extension(actor): Extension<ActorDto>,
) -> Result<JsonResponse> {
    let buckets = list_buckets(&state.db_pool, &actor.client_id).await?;
    Ok(JsonResponse::new(serde_json::to_string(&buckets).unwrap()))
}

pub async fn get_bucket_handler(Extension(bucket): Extension<Bucket>) -> Result<JsonResponse> {
    // Extract bucket from the middleware extension
    Ok(JsonResponse::new(serde_json::to_string(&bucket).unwrap()))
}
