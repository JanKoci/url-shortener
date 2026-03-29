mod errors;
mod routes;
mod utils;

use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};
use tower_http::trace::TraceLayer;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub redis: redis::aio::MultiplexedConnection,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("url_shortener=debug,tower_http=debug")
        .init();
    dotenvy::dotenv().ok();

    // connect to db
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let pool = sqlx::PgPool::connect(&db_url)
        .await
        .expect("Failed to connect to database");

    // connect to reddis
    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL not set");
    let redis_client = redis::Client::open(redis_url).expect("Invalid Redis URL");
    let redis = redis_client
        .get_multiplexed_tokio_connection()
        .await
        .expect("Failed to connect to Redis");

    let govener_config = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(2) // 1 token added every 2 seconds
            .burst_size(5) // allow bursts of up to 5 requests
            .finish()
            .unwrap(),
    );

    let state = AppState { db: pool, redis };
    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/shorten", post(routes::shorten::shorten_handler))
        .route("/stats/{code}", get(routes::stats::stats_handler))
        .route("/qr/{code}", get(routes::qr::qr_handler))
        .route("/{code}", get(routes::redirect::redirect_handler))
        .layer(TraceLayer::new_for_http())
        .layer(GovernorLayer::new(govener_config))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::info!(
        "Server started on http://{}",
        listener.local_addr().unwrap()
    );
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

async fn health_handler(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> &'static str {
    sqlx::query("SELECT 1")
        .execute(&state.db)
        .await
        .expect("DB ping failed");
    "Ok"
}
