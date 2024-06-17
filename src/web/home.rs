use crate::web::server::AppState;

use diesel::prelude::*;
use diesel::{QueryDsl, SelectableHelper};

use crate::files::models::bucket::Bucket;
use crate::schema::buckets::dsl::buckets;

use axum::{body::Body, extract::State, http::StatusCode, response::Response};

pub async fn home_handler(state: State<AppState>) -> Response<Body> {
    // Try to query buckets
    let pool = state.db_pool.clone();
    let db = pool.get().await.unwrap();

    let result = db
        .interact(|conn| {
            let results: Vec<Bucket> = buckets
                .select(Bucket::as_select())
                .load(conn)
                .expect("Error loading buckets");
            results
        })
        .await;

    match result {
        Ok(results) => {
            println!("Displaying {} posts", results.len());
            for item in results {
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
