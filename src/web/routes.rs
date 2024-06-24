use crate::web::server::AppState;

use axum::{
    middleware,
    routing::{any, get, post},
    Router,
};

use super::{
    auth::authenticate_handler,
    buckets::routes::buckets_routes,
    health::{health_live_handler, health_ready_handler},
    home::home_handler,
    middlewares::auth_middleware,
    not_found::not_found_handler,
};

pub fn all_routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(home_handler))
        .route("/health/live", get(health_live_handler))
        .route("/health/ready", get(health_ready_handler))
        .route("/v1/auth/token", post(authenticate_handler))
        .nest("/v1/buckets", buckets_routes(state.clone()))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .fallback(any(not_found_handler))
        .with_state(state)
}
