use crate::web::server::AppState;

use axum::{
    middleware,
    routing::{any, get},
    Router,
};

use super::{
    buckets::routes::buckets_routes, home::home_handler, middlewares::auth::auth_middleware,
    not_found::not_found_handler,
};

pub fn all_routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(home_handler))
        .nest("/v1/buckets", buckets_routes(state.clone()))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .fallback(any(not_found_handler))
        .with_state(state)
}
