use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use serde_json::{json, Value};

use crate::services::ollama;
use crate::state::AppState;

pub async fn health_check(
    State(state): State<Arc<AppState>>,
) -> Json<Value> {
    let db_ok = sqlx::query("SELECT 1")
        .fetch_one(&state.db)
        .await
        .is_ok();

    let ollama_ok = ollama::check_health(&state.ollama_url).await.unwrap_or(false);

    let status = if db_ok && ollama_ok {
        "healthy"
    } else if db_ok || ollama_ok {
        "degraded"
    } else {
        "unhealthy"
    };

    Json(json!({
        "status": status,
        "database": db_ok,
        "ollama": ollama_ok,
    }))
}
