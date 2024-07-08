use axum::extract::DefaultBodyLimit;
use axum::{routing::get, Router};
use tower_http::limit::RequestBodyLimitLayer;

use crate::web::server::AppState;

use super::{create_file_handler, list_files_handler};

pub fn files_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(list_files_handler).post(create_file_handler))
        .layer(DefaultBodyLimit::max(8000000))
        .layer(RequestBodyLimitLayer::new(8000000))
        .with_state(state)
}
