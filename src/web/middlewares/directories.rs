use axum::{
    body::Body,
    extract::{Path, Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};

use crate::{files::queries::directories::get_directory, web::server::AppState};

pub async fn dir_middleware(
    state: State<AppState>,
    mut request: Request,
    Path(bucket_id): Path<String>,
    Path(dir_id): Path<String>,
    next: Next,
) -> Response<Body> {
    let pool = state.db_pool.clone();
    let bid = bucket_id.clone();
    let did = dir_id.clone();
    let query_res = get_directory(pool, did.as_str()).await;
    let Ok(dir_res) = query_res else {
        return Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from("Error getting directory"))
            .unwrap();
    };

    let Some(dir) = dir_res else {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Directory not found"))
            .unwrap();
    };

    if dir.bucket_id != bid {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Directory not found"))
            .unwrap();
    }

    // Forward to the next middleware/handler passing the bucket information
    request.extensions_mut().insert(dir);
    let response = next.run(request).await;
    response
}
