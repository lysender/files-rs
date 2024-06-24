use deadpool_diesel::sqlite::{Manager, Pool, Runtime};
use dotenvy::dotenv;
use std::env;

pub fn create_db_pool() -> Pool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = Manager::new(database_url, Runtime::Tokio1);
    Pool::builder(manager).max_size(8).build().unwrap()
}
