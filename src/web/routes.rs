use crate::web::server::AppState;

use axum::{
    routing::{any, get},
    Router,
};

use super::{buckets::routes::buckets_routes, home::home_handler, not_found::not_found_handler};

pub fn all_routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(home_handler))
        .nest("/v1/buckets", buckets_routes(state.clone()))
        .fallback(any(not_found_handler))
        .with_state(state)
}
