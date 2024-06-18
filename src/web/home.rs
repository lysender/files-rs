use axum::{body::Body, http::StatusCode, response::Response};

pub async fn home_handler() -> Response<Body> {
    let r = Response::builder().status((StatusCode::OK).as_u16());
    let body = "OK".to_string();
    r.body(Body::from(body)).unwrap()
}
