mod errors;
mod routes;
mod utils;

use axum::routing::post;
use axum::{routing::get, Router};

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

    let state = AppState { db: pool };
    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/shorten", post(routes::shorten::shorten_handler))
        .route("/stats/{code}", get(routes::stats::stats_handler))
        .route("/{code}", get(routes::redirect::redirect_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
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
