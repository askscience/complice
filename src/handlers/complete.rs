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
    let sid = session_id.to_string();

    let mission = sqlx::query_as::<_, (i32, String, i32, String)>(
        "SELECT id, session_id, points, expires_at FROM missions WHERE id = ?"
    )
    .bind(body.mission_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("Mission not found".into()))?;

    if mission.1 != sid {
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

    let mut points_earned = mission.2;

    sqlx::query(
        "INSERT INTO completed_missions (mission_id, completed_at, points_earned) VALUES (?, datetime('now'), ?)"
    )
    .bind(body.mission_id)
    .bind(points_earned)
    .execute(&state.db)
    .await?;

    let remaining: i64 = sqlx::query_scalar(
        r#"SELECT COUNT(*) FROM missions
           WHERE session_id = ? AND expires_at > datetime('now')
           AND id NOT IN (SELECT mission_id FROM completed_missions)"#
    )
    .bind(&sid)
    .fetch_one(&state.db)
    .await?;

    let bonus_awarded = if remaining == 0 {
        let bonus_claimed: bool = sqlx::query_scalar(
            "SELECT bonus_claimed FROM sessions WHERE id = ?"
        )
        .bind(&sid)
        .fetch_one(&state.db)
        .await?;

        if !bonus_claimed {
            let active_count: i64 = sqlx::query_scalar(
                r#"SELECT COUNT(*) FROM missions
                   WHERE session_id = ? AND expires_at > datetime('now')"#
            )
            .bind(&sid)
            .fetch_one(&state.db)
            .await?;

            if active_count >= 3 {
                points_earned += 50;
                sqlx::query("UPDATE sessions SET bonus_claimed = 1 WHERE id = ?")
                    .bind(&sid)
                    .execute(&state.db)
                    .await?;
                true
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    };

    if bonus_awarded {
        sqlx::query(
            "UPDATE completed_missions SET points_earned = ? WHERE mission_id = ?"
        )
        .bind(points_earned)
        .bind(body.mission_id)
        .execute(&state.db)
        .await?;
    }

    let today = Utc::now().date_naive();
    let yesterday = today - Duration::days(1);

    let streak: Option<StreakRow> = sqlx::query_as(
        "SELECT session_id, current_streak, last_completion_date FROM streaks WHERE session_id = ?"
    )
    .bind(&sid)
    .fetch_optional(&state.db)
    .await?;

    let new_streak = match streak {
        Some(ref s) => match &s.last_completion_date {
            Some(date) if *date == today => s.current_streak,
            Some(date) if *date == yesterday => s.current_streak + 1,
            Some(_) => 1,
            None => 1,
        },
        None => 1,
    };

    let today_str = today.to_string();

    if streak.is_some() {
        sqlx::query("UPDATE streaks SET current_streak = ?, last_completion_date = ? WHERE session_id = ?")
            .bind(new_streak)
            .bind(&today_str)
            .bind(&sid)
            .execute(&state.db)
            .await?;
    } else {
        sqlx::query("INSERT INTO streaks (session_id, current_streak, last_completion_date) VALUES (?, ?, ?)")
            .bind(&sid)
            .bind(new_streak)
            .bind(&today_str)
            .execute(&state.db)
            .await?;
    }

    let total: TotalPointsRow = sqlx::query_as(
        "SELECT COALESCE(SUM(cm.points_earned), 0) AS total FROM completed_missions cm JOIN missions m ON m.id = cm.mission_id WHERE m.session_id = ?"
    )
    .bind(&sid)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(CompleteResponse {
        points: points_earned,
        streak: new_streak,
        total_points: total.total.unwrap_or(0),
    }))
}
