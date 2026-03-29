mod errors;
mod routes;
mod utils;

use std::net::SocketAddr;
use std::sync::Arc;
use axum::routing::post;
use axum::{routing::get, Router};
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let pool = sqlx::PgPool::connect(&db_url)
        .await
        .expect("Failed to connect to database");
    
    let govener_config = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(2) // 1 token added every 2 seconds
            .burst_size(5) // allow bursts of up to 5 requests
            .finish()
            .unwrap(),
    );

    let state = AppState { db: pool };
    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/shorten", post(routes::shorten::shorten_handler))
        .route("/stats/{code}", get(routes::stats::stats_handler))
        .route("/{code}", get(routes::redirect::redirect_handler))
        .layer(GovernorLayer::new(govener_config))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
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
