use axum::{
    body::Body,
    extract::{rejection::JsonRejection, Json, Path, State},
    http::StatusCode,
    response::Response,
    Extension,
};

use crate::{
    files::{
        models::{Dir, NewDir, UpdateDir},
        queries::dirs::{create_dir, delete_dir, get_dir, list_dirs, update_dir},
    },
    web::{
        params::Params,
        response::{
            create_error_response, create_response, create_success_response, to_error_response,
        },
        server::AppState,
    },
};

pub async fn list_dirs_handler(
    State(state): State<AppState>,
    Path(bucket_id): Path<String>,
) -> Response<Body> {
    let res = list_dirs(&state.db_pool, &bucket_id).await;
    let Ok(dirs) = res else {
        return create_error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to list directories".to_string(),
            "Internal Server Error".to_string(),
        );
    };

    create_success_response(serde_json::to_string(&dirs).unwrap())
}

pub async fn create_dir_handler(
    State(state): State<AppState>,
    Path(bucket_id): Path<String>,
    Json(payload): Json<NewDir>,
) -> Response<Body> {
    let res = create_dir(&state.db_pool, &bucket_id, &payload).await;
    match res {
        Ok(dir) => create_response(StatusCode::CREATED, serde_json::to_string(&dir).unwrap()),
        Err(error) => to_error_response(error),
    }
}

pub async fn get_dir_handler(Extension(dir): Extension<Dir>) -> Response<Body> {
    // Extract dir from the middleware extension
    return create_success_response(serde_json::to_string(&dir).unwrap());
}

pub async fn update_dir_handler(
    State(state): State<AppState>,
    Extension(dir): Extension<Dir>,
    Path(params): Path<Params>,
    payload: Option<Json<UpdateDir>>,
) -> Response<Body> {
    let dir_id = params.dir_id.clone().expect("dir_id is required");
    let Some(data) = payload else {
        return create_error_response(
            StatusCode::BAD_REQUEST,
            "Invalid request payload".to_string(),
            "Bad Request".to_string(),
        );
    };

    let res = update_dir(&state.db_pool, &dir_id, &data).await;
    match res {
        Ok(updated) => {
            if updated {
                get_dir_as_response(&state, &dir_id).await
            } else {
                // Just send back the original dir
                create_success_response(serde_json::to_string(&dir).unwrap())
            }
        }
        Err(error) => to_error_response(error),
    }
}

async fn get_dir_as_response(state: &AppState, id: &str) -> Response<Body> {
    let res = get_dir(&state.db_pool, id).await;
    let Ok(dir_res) = res else {
        return create_error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error getting directory".to_string(),
            "Internal Server Error".to_string(),
        );
    };

    let Some(dir) = dir_res else {
        return create_error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error getting directory, directory not found".to_string(),
            "Internal Server Error".to_string(),
        );
    };

    create_success_response(serde_json::to_string(&dir).unwrap())
}

pub async fn delete_dir_handler(
    State(state): State<AppState>,
    Path(dir_id): Path<String>,
) -> Response<Body> {
    let res = delete_dir(&state.db_pool, &dir_id).await;
    match res {
        Ok(_) => create_response(StatusCode::NO_CONTENT, "".to_string()),
        Err(error) => to_error_response(error),
    }
}
