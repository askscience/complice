use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use uuid::Uuid;

use crate::error::ApiError;
use crate::models::{RerollRequest, RerollResponse};
use crate::state::AppState;

use super::generate::core_generate;

pub async fn reroll_mission(
    State(state): State<Arc<AppState>>,
    Json(body): Json<RerollRequest>,
) -> Result<Json<RerollResponse>, ApiError> {
    let session_id = Uuid::parse_str(&body.session_id)?;

    let mission = sqlx::query_as::<_, (String,)>(
        "SELECT session_id FROM missions WHERE id = ?"
    )
    .bind(body.mission_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("Mission not found".into()))?;

    if mission.0 != session_id.to_string() {
        return Err(ApiError::BadRequest("Mission does not belong to this session".into()));
    }

    let completed: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM completed_missions WHERE mission_id = ?)"
    )
    .bind(body.mission_id)
    .fetch_one(&state.db)
    .await?;

    if completed {
        return Err(ApiError::BadRequest("Cannot reroll a completed mission".into()));
    }

    sqlx::query("DELETE FROM missions WHERE id = ?")
        .bind(body.mission_id)
        .execute(&state.db)
        .await?;

    let missions = core_generate(
        &state,
        session_id,
        &body.interests,
        &body.location,
        body.mood.as_deref(),
        body.model.as_deref(),
        body.difficulty.as_deref(),
        1,
    )
    .await?;

    let mission = missions.into_iter().next()
        .ok_or_else(|| ApiError::Internal("Failed to generate replacement mission".into()))?;

    Ok(Json(RerollResponse { mission }))
}
