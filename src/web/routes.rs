use crate::web::server::AppState;

use axum::{
    body::Body,
    extract::State,
    http::StatusCode,
    response::Response,
    routing::{any, get},
    Router,
};

pub fn all_routes(state: AppState) -> Router {
    // Route root and fallback to the same proxy handler
    Router::new()
        .route("/", get(home_handler))
        .fallback(any(not_found_handler))
        .with_state(state)
}

async fn home_handler(_state: State<AppState>) -> Response<Body> {
    let mut r = Response::builder().status((StatusCode::OK).as_u16());
    let body = "OK".to_string();
    r.body(Body::from(body)).unwrap()
}

async fn not_found_handler(_state: State<AppState>) -> Response<Body> {
    let mut r = Response::builder().status((StatusCode::NOT_FOUND).as_u16());
    let body = "Not Found".to_string();
    r.body(Body::from(body)).unwrap()
}
