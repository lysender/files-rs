use crate::files::queries::buckets::list_buckets;
use crate::web::server::AppState;

use axum::{body::Body, extract::State, http::StatusCode, response::Response};

pub async fn home_handler(state: State<AppState>) -> Response<Body> {
    // Try to query buckets
    let pool = state.db_pool.clone();
    let result = list_buckets(pool, "client_id").await;

    match result {
        Ok(buckets) => {
            println!("Displaying {} posts", buckets.len());
            for item in buckets {
                println!("{}", item.id);
                println!("-----------\n");
                println!("{}", item.name);
            }
        }
        Err(e) => {
            eprintln!("{e}");
        }
    }

    let r = Response::builder().status((StatusCode::OK).as_u16());
    let body = "OK".to_string();
    r.body(Body::from(body)).unwrap()
}
