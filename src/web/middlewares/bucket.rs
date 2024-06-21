use axum::{
    body::Body,
    extract::{Path, Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};

use crate::{
    files::queries::buckets::get_bucket,
    util::valid_id,
    web::{params::Params, response::create_error_response, server::AppState},
};

pub async fn bucket_middleware(
    State(state): State<AppState>,
    Path(params): Path<Params>,
    mut request: Request,
    next: Next,
) -> Response<Body> {
    if !valid_id(&params.bucket_id) {
        return create_error_response(
            StatusCode::BAD_REQUEST,
            "Invalid bucket id".to_string(),
            "Bad Request".to_string(),
        );
    }

    let query_res = get_bucket(&state.db_pool, &params.bucket_id).await;
    let Ok(bucket_res) = query_res else {
        return create_error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error getting bucket".to_string(),
            "Internal Server Error".to_string(),
        );
    };

    let Some(bucket) = bucket_res else {
        return create_error_response(
            StatusCode::NOT_FOUND,
            "Bucket not found".to_string(),
            "Not Found".to_string(),
        );
    };

    // Forward to the next middleware/handler passing the bucket information
    request.extensions_mut().insert(bucket);
    let response = next.run(request).await;
    response
}
