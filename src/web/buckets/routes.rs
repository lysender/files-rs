use axum::{middleware, routing::get, Router};

use crate::web::{
    dirs::routes::dir_routes,
    middlewares::{auth::auth_middleware, bucket::bucket_middleware},
    server::AppState,
};

use super::handlers::{
    create_bucket_handler, delete_bucket_handler, get_bucket_handler, list_buckets_handler,
    update_bucket_handler,
};

pub fn buckets_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(list_buckets_handler).post(create_bucket_handler))
        .nest("/:bucket_id", inner_bucket_routes(state.clone()))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(state)
}

fn inner_bucket_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(get_bucket_handler)
                .patch(update_bucket_handler)
                .delete(delete_bucket_handler),
        )
        .nest("/dirs", dir_routes(state.clone()))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            bucket_middleware,
        ))
        .with_state(state)
}
