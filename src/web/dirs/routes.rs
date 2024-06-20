use axum::{
    middleware,
    routing::{delete, get, patch},
    Router,
};

use crate::web::{middlewares::dir::dir_middleware, server::AppState};

use super::handlers::{
    create_dir_handler, delete_dir_handler, get_dir_handler, list_dirs_handler, update_dir_handler,
};

pub fn dir_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(list_dirs_handler).post(create_dir_handler))
        .nest("/:dir_id", inner_dir_routes(state.clone()))
        .with_state(state)
}

fn inner_dir_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(get_dir_handler))
        .route("/", patch(update_dir_handler))
        .route("/", delete(delete_dir_handler))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            dir_middleware,
        ))
        .with_state(state)
}
