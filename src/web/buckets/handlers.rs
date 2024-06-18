use axum::{body::Body, extract::State, http::StatusCode, response::Response};

use crate::web::server::AppState;

pub async fn list_buckets_handler(State(_state): State<AppState>) -> Response<Body> {
    let r = Response::builder().status((StatusCode::OK).as_u16());
    let body = "Home page".to_string();
    r.body(Body::from(body)).unwrap()
}

pub async fn create_bucket_handler(State(_state): State<AppState>) -> Response<Body> {
    let r = Response::builder().status((StatusCode::OK).as_u16());
    let body = "Home page".to_string();
    r.body(Body::from(body)).unwrap()
}

pub async fn get_bucket_handler(State(_state): State<AppState>) -> Response<Body> {
    let r = Response::builder().status((StatusCode::OK).as_u16());
    let body = "Home page".to_string();
    r.body(Body::from(body)).unwrap()
}

pub async fn update_bucket_handler(State(_state): State<AppState>) -> Response<Body> {
    let r = Response::builder().status((StatusCode::OK).as_u16());
    let body = "Home page".to_string();
    r.body(Body::from(body)).unwrap()
}

pub async fn delete_bucket_handler(State(_state): State<AppState>) -> Response<Body> {
    let r = Response::builder().status((StatusCode::OK).as_u16());
    let body = "Home page".to_string();
    r.body(Body::from(body)).unwrap()
}
