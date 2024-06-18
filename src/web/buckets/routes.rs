use axum::{
    routing::{delete, get, patch},
    Router,
};

use crate::web::server::AppState;

use super::handlers::{
    create_bucket_handler, delete_bucket_handler, get_bucket_handler, list_buckets_handler,
    update_bucket_handler,
};

pub fn buckets_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(list_buckets_handler).post(create_bucket_handler))
        .nest("/:bucket_id", inner_bucket_routes(state.clone()))
        .with_state(state)
}

fn inner_bucket_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(get_bucket_handler))
        .route("/", patch(update_bucket_handler))
        .route("/", delete(delete_bucket_handler))
        .with_state(state)
}
