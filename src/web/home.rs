use axum::{body::Body, extract::State, http::StatusCode, response::Response};
use serde::Serialize;

use super::server::AppState;

#[derive(Serialize)]
pub struct AppMeta {
    pub name: String,
    pub version: String,
}

pub async fn home_handler(State(_state): State<AppState>) -> Response<Body> {
    let r = Response::builder().status((StatusCode::OK).as_u16());
    let body = "Home page".to_string();
    r.body(Body::from(body)).unwrap()
}
