use crate::web::server::AppState;

use axum::{
    routing::{any, get},
    Router,
};

use super::{home::home_handler, not_found::not_found_handler};

pub fn all_routes(state: AppState) -> Router {
    // Route root and fallback to the same proxy handler
    Router::new()
        .route("/", get(home_handler))
        .fallback(any(not_found_handler))
        .with_state(state)
}
