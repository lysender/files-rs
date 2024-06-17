use crate::files::models::NewBucket;
use crate::files::queries::buckets::create_bucket;
use crate::files::queries::buckets::list_buckets;
use crate::web::server::AppState;

use axum::{body::Body, extract::State, http::StatusCode, response::Response};

pub async fn home_handler(state: State<AppState>) -> Response<Body> {
    // Try to query buckets
    let pool = state.db_pool.clone();
    let client_id = "019026f577437f47a9fa888e1d4d3e25".to_string();

    let bucket_data = NewBucket {
        name: "photos-dev".to_string(),
        label: "photos-dev".to_string(),
    };
    let insert_res = create_bucket(pool.clone(), client_id.as_str(), bucket_data).await;
    match insert_res {
        Ok(new_bucket) => {
            println!("Bucket created");
            println!("{}", new_bucket.id);
        }
        Err(e) => {
            eprintln!("{e}");
        }
    }

    let result = list_buckets(pool, client_id.as_str()).await;

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
