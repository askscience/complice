use std::sync::Arc;

use axum::extract::{Query, State};
use axum::Json;
use serde::Deserialize;
use uuid::Uuid;

use crate::error::ApiError;
use crate::models::{CompletedMissionInfo, StatsResponse, StreakRow, TotalPointsRow};
use crate::state::AppState;

#[derive(Deserialize)]
pub struct StatsQuery {
    pub session_id: String,
}

pub async fn get_stats(
    State(state): State<Arc<AppState>>,
    Query(query): Query<StatsQuery>,
) -> Result<Json<StatsResponse>, ApiError> {
    let session_id = Uuid::parse_str(&query.session_id)?;
    let sid = session_id.to_string();

    let total: TotalPointsRow = sqlx::query_as(
        "SELECT COALESCE(SUM(cm.points_earned), 0) AS total FROM completed_missions cm JOIN missions m ON m.id = cm.mission_id WHERE m.session_id = ?"
    )
    .bind(&sid)
    .fetch_one(&state.db)
    .await?;

    let streak: Option<StreakRow> = sqlx::query_as(
        "SELECT session_id, current_streak, last_completion_date FROM streaks WHERE session_id = ?"
    )
    .bind(&sid)
    .fetch_optional(&state.db)
    .await?;

    let current_streak = streak.as_ref().map(|s| s.current_streak).unwrap_or(0);

    let completed_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM completed_missions cm JOIN missions m ON m.id = cm.mission_id WHERE m.session_id = ?"
    )
    .bind(&sid)
    .fetch_one(&state.db)
    .await?;

    let mut badges = Vec::new();
    let pts = total.total.unwrap_or(0);

    if current_streak >= 3 {
        badges.push("Hot Streak".to_string());
    }
    if current_streak >= 7 {
        badges.push("On Fire".to_string());
    }
    if current_streak >= 30 {
        badges.push("Unstoppable".to_string());
    }
    if pts >= 100 {
        badges.push("Centurion".to_string());
    }
    if pts >= 500 {
        badges.push("High Scorer".to_string());
    }
    if pts >= 1000 {
        badges.push("Legend".to_string());
    }
    if completed_count >= 10 {
        badges.push("Explorer".to_string());
    }
    if completed_count >= 50 {
        badges.push("Adventurer".to_string());
    }

    let last_completed: Vec<CompletedMissionInfo> = sqlx::query_as(
        r#"SELECT m.id AS mission_id, m.title, m.description,
                  cm.points_earned, cm.completed_at
           FROM completed_missions cm
           JOIN missions m ON m.id = cm.mission_id
           WHERE m.session_id = ?
           ORDER BY cm.completed_at DESC
           LIMIT 5"#
    )
    .bind(&sid)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(StatsResponse {
        total_points: pts,
        current_streak,
        badges,
        last_completed,
    }))
}
