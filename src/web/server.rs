use axum::extract::{DefaultBodyLimit, FromRef};
use axum::Router;
use deadpool_diesel::sqlite::Pool;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::{info, Level};

use crate::config::Config;
use crate::db::create_db_pool;
use crate::web::routes::all_routes;
use crate::Result;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub config: Config,
    pub db_pool: Pool,
}

pub async fn run_web_server(config: Config) -> Result<()> {
    let port = config.server.port;

    let pool = create_db_pool();
    let state = AppState {
        config,
        db_pool: pool,
    };

    let mut routes_all = Router::new()
        .merge(all_routes(state))
        .layer(DefaultBodyLimit::max(8000000))
        .layer(RequestBodyLimitLayer::new(8000000));

    routes_all = routes_all.layer(
        ServiceBuilder::new().layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        ),
    );

    // Setup the server
    let ip = "127.0.0.1";
    let addr = format!("{}:{}", ip, port);
    info!("HTTP server running on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}
