mod cache;
mod config;
mod error;
mod fallback;
mod handlers;
mod models;
mod services;
mod state;

use std::sync::Arc;

use axum::{Router, routing::{get, post}};
use sqlx::SqlitePool;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::{ServeDir, ServeFile};
use tracing_subscriber::EnvFilter;

use crate::config::Config;
use crate::state::AppState;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("info")))
        .init();

    let config = Config::from_env();

    let pool = SqlitePool::connect(&config.database_url)
        .await
        .expect("Failed to connect to SQLite");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    tracing::info!("Database migrations complete");

    let state = Arc::new(AppState {
        db: pool,
        ollama_url: config.ollama_url,
        default_model: config.default_model,
        weather_api_endpoint: config.weather_api_endpoint,
        cache: cache::Cache::new(),
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/health", get(handlers::health::health_check))
        .route("/api/models", get(handlers::models::get_models))
        .route("/api/generate", post(handlers::generate::generate_missions))
        .route("/api/complete", post(handlers::complete::complete_mission))
        .route("/api/stats", get(handlers::stats::get_stats))
        .layer(cors)
        .with_state(state)
        .fallback_service(ServeDir::new("web").not_found_service(ServeFile::new("web/index.html")));

    let addr = format!("0.0.0.0:{}", config.port);
    tracing::info!("Complice server starting on {addr}");

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}
