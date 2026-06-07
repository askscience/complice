use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use chrono::{Duration, Utc};
use uuid::Uuid;

use crate::error::ApiError;
use crate::fallback;
use crate::models::{
    GenerateRequest, GenerateResponse, MissionResponse, MissionData,
};
use crate::services::{ollama, weather, prompt};
use crate::state::AppState;

pub async fn generate_missions(
    State(state): State<Arc<AppState>>,
    Json(body): Json<GenerateRequest>,
) -> Result<Json<GenerateResponse>, ApiError> {
    let session_id = match &body.session_id {
        Some(id) => {
            let uuid = Uuid::parse_str(id)?;
            sqlx::query("UPDATE sessions SET last_active = datetime('now') WHERE id = ?")
                .bind(uuid.to_string())
                .execute(&state.db)
                .await?;
            uuid
        }
        None => {
            let uuid = Uuid::new_v4();
            sqlx::query("INSERT INTO sessions (id, created_at, last_active) VALUES (?, datetime('now'), datetime('now'))")
                .bind(uuid.to_string())
                .execute(&state.db)
                .await?;
            uuid
        }
    };

    let weather_cache_key = format!("weather:{}:{}", body.location.lat, body.location.lon);
    let weather_summary = if let Some(cached) = state.cache.get(&weather_cache_key) {
        cached
    } else {
        let summary = weather::get_weather(&state.weather_api_endpoint, &body.location)
            .await
            .unwrap_or_else(|e| {
                tracing::warn!("Weather fetch failed: {e:?}");
                "clear, mild".to_string()
            });
        state.cache.set(&weather_cache_key, summary.clone(), 300);
        summary
    };

    let now = Utc::now();
    let seven_days_ago = (now - Duration::days(7)).to_rfc3339();

    let recent: Vec<String> = sqlx::query_scalar(
        r#"SELECT m.title
           FROM completed_missions cm
           JOIN missions m ON m.id = cm.mission_id
           WHERE m.session_id = ? AND cm.completed_at > ?
           ORDER BY cm.completed_at DESC
           LIMIT 10"#
    )
    .bind(session_id.to_string())
    .bind(&seven_days_ago)
    .fetch_all(&state.db)
    .await?;

    let prompt = prompt::build_prompt(
        &body.interests,
        &weather_summary,
        body.mood.as_deref(),
        &recent,
    );

    let model = body.model.as_deref().unwrap_or(&state.default_model);

    let missions_data: Vec<MissionData> = match ollama::generate_missions(
        &state.ollama_url,
        model,
        &prompt,
    )
    .await
    {
        Ok(missions) if !missions.is_empty() => missions,
        _ => {
            tracing::warn!("Ollama generation failed, using fallback missions");
            fallback::get_fallback_missions()
        }
    };

    let mut saved_missions = Vec::new();
    let now_str = now.to_rfc3339();
    let expires_str = (now + Duration::hours(24)).to_rfc3339();

    for mission in &missions_data {
        let row = sqlx::query_as::<_, (i32,)>(
            r#"INSERT INTO missions
               (session_id, title, description, time_limit_minutes, radius_meters, points, generated_at, expires_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?)
               RETURNING id"#
        )
        .bind(session_id.to_string())
        .bind(&mission.title)
        .bind(&mission.description)
        .bind(mission.time_limit_minutes)
        .bind(mission.radius_meters)
        .bind(mission.points)
        .bind(&now_str)
        .bind(&expires_str)
        .fetch_one(&state.db)
        .await?;

        saved_missions.push(MissionResponse {
            id: row.0,
            title: mission.title.clone(),
            description: mission.description.clone(),
            time_limit_minutes: mission.time_limit_minutes,
            radius_meters: mission.radius_meters,
            points: mission.points,
        });
    }

    Ok(Json(GenerateResponse {
        session_id: session_id.to_string(),
        missions: saved_missions,
    }))
}
