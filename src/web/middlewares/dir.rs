use axum::{
    body::Body,
    extract::{Path, Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};

use crate::{
    files::queries::dirs::get_dir,
    web::{params::Params, response::create_error_response, server::AppState},
};

pub async fn dir_middleware(
    state: State<AppState>,
    Path(params): Path<Params>,
    mut request: Request,
    next: Next,
) -> Response<Body> {
    let did = params.dir_id.clone().expect("dir_id is required");
    let query_res = get_dir(&state.db_pool, &did).await;
    let Ok(dir_res) = query_res else {
        return create_error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error getting directory".to_string(),
            "Internal Server Error".to_string(),
        );
    };

    let Some(dir) = dir_res else {
        return create_error_response(
            StatusCode::NOT_FOUND,
            "Directory not found".to_string(),
            "Not Found".to_string(),
        );
    };

    if dir.bucket_id != params.bucket_id.clone() {
        return create_error_response(
            StatusCode::NOT_FOUND,
            "Directory not found".to_string(),
            "Not Found".to_string(),
        );
    }

    // Forward to the next middleware/handler passing the bucket information
    request.extensions_mut().insert(dir);
    let response = next.run(request).await;
    response
}
