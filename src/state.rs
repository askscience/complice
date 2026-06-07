use sqlx::SqlitePool;

use crate::cache::Cache;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub ollama_url: String,
    pub default_model: String,
    pub weather_api_endpoint: String,
    pub cache: Cache,
}
