use crate::web::server::AppState;

use axum::{body::Body, extract::State, http::StatusCode, response::Response};

pub async fn not_found_handler(_state: State<AppState>) -> Response<Body> {
    let r = Response::builder().status((StatusCode::NOT_FOUND).as_u16());
    let body = "Not Found".to_string();
    r.body(Body::from(body)).unwrap()
}
