use axum::{body::Body, http::StatusCode, response::Response};

pub async fn list_buckets_handler() -> Response<Body> {
    let r = Response::builder().status((StatusCode::OK).as_u16());
    let body = "buckets listing".to_string();
    r.body(Body::from(body)).unwrap()
}

pub async fn create_bucket_handler() -> Response<Body> {
    let r = Response::builder().status((StatusCode::OK).as_u16());
    let body = "create new bucket".to_string();
    r.body(Body::from(body)).unwrap()
}

pub async fn get_bucket_handler() -> Response<Body> {
    let r = Response::builder().status((StatusCode::OK).as_u16());
    let body = "get bucket".to_string();
    r.body(Body::from(body)).unwrap()
}

pub async fn update_bucket_handler() -> Response<Body> {
    let r = Response::builder().status((StatusCode::OK).as_u16());
    let body = "update bucket".to_string();
    r.body(Body::from(body)).unwrap()
}

pub async fn delete_bucket_handler() -> Response<Body> {
    let r = Response::builder().status((StatusCode::OK).as_u16());
    let body = "delete bucket".to_string();
    r.body(Body::from(body)).unwrap()
}
