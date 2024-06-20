use axum::{
    body::Body,
    extract::{Json, Path, State},
    http::StatusCode,
    response::Response,
    Extension,
};

use crate::{
    files::{
        models::{Bucket, NewBucket, UpdateBucket},
        queries::buckets::{create_bucket, delete_bucket, get_bucket, list_buckets, update_bucket},
    },
    web::{
        response::{
            create_error_response, create_response, create_success_response, to_error_response,
        },
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

    let bucket_res = create_bucket(pool, client_id.as_str(), payload).await;
    match bucket_res {
        Ok(bucket) => create_response(StatusCode::CREATED, serde_json::to_string(&bucket).unwrap()),
        Err(error) => to_error_response(error),
    }
}

pub async fn get_bucket_handler(Extension(bucket): Extension<Bucket>) -> Response<Body> {
    // Extract bucket from the middleware extension
    return create_success_response(serde_json::to_string(&bucket).unwrap());
}

pub async fn update_bucket_handler(
    State(state): State<AppState>,
    Extension(bucket): Extension<Bucket>,
    Path(bucket_id): Path<String>,
    Json(payload): Json<UpdateBucket>,
) -> Response<Body> {
    let pool = state.db_pool.clone();
    let bucket_res = update_bucket(pool, bucket_id.as_str(), &payload).await;
    match bucket_res {
        Ok(updated) => {
            if updated {
                get_bucket_as_response(&state, bucket_id.as_str()).await
            } else {
                // Just send back the original bucket
                create_success_response(serde_json::to_string(&bucket).unwrap())
            }
        }
        Err(error) => to_error_response(error),
    }
}

async fn get_bucket_as_response(state: &AppState, id: &str) -> Response<Body> {
    let query_res = get_bucket(state.db_pool.clone(), id).await;
    let Ok(bucket_res) = query_res else {
        return create_error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error getting bucket".to_string(),
            "Internal Server Error".to_string(),
        );
    };

    let Some(bucket) = bucket_res else {
        return create_error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error getting bucket, bucket not found".to_string(),
            "Internal Server Error".to_string(),
        );
    };

    create_success_response(serde_json::to_string(&bucket).unwrap())
}

pub async fn delete_bucket_handler(
    State(state): State<AppState>,
    Path(bucket_id): Path<String>,
) -> Response<Body> {
    let pool = state.db_pool.clone();
    let bucket_res = delete_bucket(pool, bucket_id.as_str()).await;
    match bucket_res {
        Ok(_) => create_response(StatusCode::NO_CONTENT, "".to_string()),
        Err(error) => to_error_response(error),
    }
}
