use std::sync::Arc;

use axum::extract::State;
use axum::Json;

use crate::error::ApiError;
use crate::services::ollama;
use crate::state::AppState;

pub async fn get_models(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<String>>, ApiError> {
    if let Some(cached) = state.cache.get("models") {
        let models: Vec<String> = serde_json::from_str(&cached)?;
        return Ok(Json(models));
    }

    let models = ollama::get_models(&state.ollama_url).await?;

    if let Ok(json) = serde_json::to_string(&models) {
        state.cache.set("models", json, 300);
    }

    Ok(Json(models))
}
