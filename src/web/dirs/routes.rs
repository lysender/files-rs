use axum::extract::DefaultBodyLimit;
use axum::{middleware, routing::get, Router};
use tower_http::limit::RequestBodyLimitLayer;

use crate::web::{middlewares::dir_middleware, server::AppState};

use super::{
    handlers::{
        create_dir_handler, delete_dir_handler, get_dir_handler, list_dirs_handler,
        update_dir_handler,
    },
    list_files_handler,
};

pub fn dir_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(list_dirs_handler).post(create_dir_handler))
        .nest("/:dir_id", inner_dir_routes(state.clone()))
        .with_state(state)
}

fn inner_dir_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(get_dir_handler)
                .patch(update_dir_handler)
                .delete(delete_dir_handler),
        )
        .nest("/files", files_routes(state.clone()))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            dir_middleware,
        ))
        .with_state(state)
}

fn files_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(list_files_handler))
        .layer(DefaultBodyLimit::max(8000000))
        .layer(RequestBodyLimitLayer::new(8000000))
        .with_state(state)
}
