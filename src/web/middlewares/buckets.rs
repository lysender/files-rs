use axum::{
    body::Body,
    extract::{Path, Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};

use crate::{files::queries::buckets::get_bucket, web::server::AppState};

pub async fn bucket_middleware(
    state: State<AppState>,
    mut request: Request,
    Path(bucket_id): Path<String>,
    next: Next,
) -> Response<Body> {
    let pool = state.db_pool.clone();
    let bid = bucket_id.clone();
    let query_res = get_bucket(pool, bid.as_str()).await;
    let Ok(bucket_res) = query_res else {
        return Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from("Error getting bucket"))
            .unwrap();
    };

    let Some(bucket) = bucket_res else {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Bucket not found"))
            .unwrap();
    };

    // Forward to the next middleware/handler passing the bucket information
    request.extensions_mut().insert(bucket);
    let response = next.run(request).await;
    response
}
