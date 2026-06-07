use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use chrono::{Duration, Utc};
use uuid::Uuid;

use crate::error::ApiError;
use crate::models::{CompleteRequest, CompleteResponse, StreakRow, TotalPointsRow};
use crate::state::AppState;

pub async fn complete_mission(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CompleteRequest>,
) -> Result<Json<CompleteResponse>, ApiError> {
    let session_id = Uuid::parse_str(&body.session_id)?;

    let mission = sqlx::query_as::<_, (i32, String, i32, String)>(
        "SELECT id, session_id, points, expires_at FROM missions WHERE id = ?"
    )
    .bind(body.mission_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("Mission not found".into()))?;

    if mission.1 != session_id.to_string() {
        return Err(ApiError::BadRequest("Mission does not belong to this session".into()));
    }

    let now = Utc::now();
    let expires_at: chrono::DateTime<Utc> = chrono::DateTime::parse_from_rfc3339(&mission.3)
        .map(|d| d.with_timezone(&Utc))
        .unwrap_or(now);
    if now > expires_at {
        return Err(ApiError::BadRequest("Mission has expired".into()));
    }

    let already_completed: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM completed_missions WHERE mission_id = ?)"
    )
    .bind(body.mission_id)
    .fetch_one(&state.db)
    .await?;

    if already_completed {
        return Err(ApiError::BadRequest("Mission already completed".into()));
    }

    sqlx::query(
        "INSERT INTO completed_missions (mission_id, completed_at, points_earned) VALUES (?, datetime('now'), ?)"
    )
    .bind(body.mission_id)
    .bind(mission.2)
    .execute(&state.db)
    .await?;

    let today = Utc::now().date_naive();
    let yesterday = today - Duration::days(1);

    let streak: Option<StreakRow> = sqlx::query_as(
        "SELECT session_id, current_streak, last_completion_date FROM streaks WHERE session_id = ?"
    )
    .bind(session_id.to_string())
    .fetch_optional(&state.db)
    .await?;

    let new_streak = match streak {
        Some(ref s) => {
            match &s.last_completion_date {
                Some(date) if *date == today => s.current_streak,
                Some(date) if *date == yesterday => s.current_streak + 1,
                Some(_) => 1,
                None => 1,
            }
        }
        None => 1,
    };

    let today_str = today.to_string();

    if streak.is_some() {
        sqlx::query(
            "UPDATE streaks SET current_streak = ?, last_completion_date = ? WHERE session_id = ?"
        )
        .bind(new_streak)
        .bind(&today_str)
        .bind(session_id.to_string())
        .execute(&state.db)
        .await?;
    } else {
        sqlx::query(
            "INSERT INTO streaks (session_id, current_streak, last_completion_date) VALUES (?, ?, ?)"
        )
        .bind(session_id.to_string())
        .bind(new_streak)
        .bind(&today_str)
        .execute(&state.db)
        .await?;
    }

    let total: TotalPointsRow = sqlx::query_as(
        "SELECT COALESCE(SUM(cm.points_earned), 0) AS total FROM completed_missions cm JOIN missions m ON m.id = cm.mission_id WHERE m.session_id = ?"
    )
    .bind(session_id.to_string())
    .fetch_one(&state.db)
    .await?;

    Ok(Json(CompleteResponse {
        points: mission.2,
        streak: new_streak,
        total_points: total.total.unwrap_or(0),
    }))
}
