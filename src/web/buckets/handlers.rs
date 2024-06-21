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
    let res = list_buckets(&state.db_pool, &state.config.client_id).await;
    let Ok(buckets) = res else {
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
    payload: Option<Json<NewBucket>>,
) -> Response<Body> {
    let Some(data) = payload else {
        return create_error_response(
            StatusCode::BAD_REQUEST,
            "Invalid request payload".to_string(),
            "Bad Request".to_string(),
        );
    };
    let res = create_bucket(&state.db_pool, &state.config.client_id, &data).await;
    match res {
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
    payload: Option<Json<UpdateBucket>>,
) -> Response<Body> {
    let Some(data) = payload else {
        return create_error_response(
            StatusCode::BAD_REQUEST,
            "Invalid request payload".to_string(),
            "Bad Request".to_string(),
        );
    };
    let res = update_bucket(&state.db_pool, &bucket_id, &data).await;
    match res {
        Ok(updated) => {
            if updated {
                get_bucket_as_response(&state, &bucket_id).await
            } else {
                // Just send back the original bucket
                create_success_response(serde_json::to_string(&bucket).unwrap())
            }
        }
        Err(error) => to_error_response(error),
    }
}

async fn get_bucket_as_response(state: &AppState, id: &str) -> Response<Body> {
    let res = get_bucket(&state.db_pool, id).await;
    let Ok(bucket_res) = res else {
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
    let res = delete_bucket(&state.db_pool, &bucket_id).await;
    match res {
        Ok(_) => create_response(StatusCode::NO_CONTENT, "".to_string()),
        Err(error) => to_error_response(error),
    }
}
