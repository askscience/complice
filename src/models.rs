use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize)]
pub struct GenerateRequest {
    pub session_id: Option<String>,
    pub interests: Vec<String>,
    pub location: Location,
    pub mood: Option<String>,
    pub model: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Location {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MissionData {
    pub title: String,
    pub description: String,
    pub time_limit_minutes: i32,
    pub radius_meters: i32,
    pub points: i32,
}

#[derive(Debug, Serialize)]
pub struct GenerateResponse {
    pub session_id: String,
    pub missions: Vec<MissionResponse>,
}

#[derive(Debug, Serialize, Clone)]
pub struct MissionResponse {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub time_limit_minutes: i32,
    pub radius_meters: i32,
    pub points: i32,
}

#[derive(Debug, Deserialize)]
pub struct CompleteRequest {
    pub session_id: String,
    pub mission_id: i32,
}

#[derive(Debug, Serialize)]
pub struct CompleteResponse {
    pub points: i32,
    pub streak: i32,
    pub total_points: i64,
}

#[derive(Debug, Serialize)]
pub struct StatsResponse {
    pub total_points: i64,
    pub current_streak: i32,
    pub badges: Vec<String>,
    pub last_completed: Vec<CompletedMissionInfo>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct CompletedMissionInfo {
    pub mission_id: i32,
    pub title: String,
    pub description: String,
    pub points_earned: i32,
    pub completed_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
#[allow(dead_code)]
pub struct StreakRow {
    pub session_id: String,
    pub current_streak: i32,
    pub last_completion_date: Option<NaiveDate>,
}

#[derive(Debug, FromRow)]
pub struct TotalPointsRow {
    pub total: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct OllamaGenerateRequest {
    pub model: String,
    pub prompt: String,
    pub stream: bool,
    pub format: String,
}

#[derive(Debug, Deserialize)]
pub struct OllamaGenerateResponse {
    pub response: String,
    #[allow(dead_code)]
    pub done: bool,
}

#[derive(Debug, Deserialize)]
pub struct OllamaMissionsResponse {
    pub missions: Vec<MissionData>,
}

#[derive(Debug, Deserialize)]
pub struct OllamaTagsResponse {
    pub models: Vec<OllamaModelInfo>,
}

#[derive(Debug, Deserialize)]
pub struct OllamaModelInfo {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct OpenMeteoResponse {
    pub current_weather: Option<CurrentWeather>,
}

#[derive(Debug, Deserialize)]
pub struct CurrentWeather {
    pub temperature: f64,
    pub weathercode: i32,
    pub windspeed: f64,
}
