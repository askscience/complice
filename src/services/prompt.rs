use chrono::{Local, Timelike};

pub fn build_prompt(
    interests: &[String],
    weather: &str,
    mood: Option<&str>,
    recent_missions: &[String],
    difficulty: Option<&str>,
    count: usize,
) -> String {
    let time_of_day = match Local::now().hour() {
        5..=11 => "morning",
        12..=16 => "afternoon",
        17..=20 => "evening",
        _ => "night",
    };

    let interests_str = if interests.is_empty() {
        "general outdoor activities".to_string()
    } else {
        interests.join(", ")
    };

    let mood_str = match mood {
        Some(m) if !m.is_empty() => format!("Current mood: {m}"),
        _ => String::new(),
    };

    let recent_str = if recent_missions.is_empty() {
        "No recent missions completed.".to_string()
    } else {
        format!(
            "Recently completed missions: {}",
            recent_missions.join(", ")
        )
    };

    let (diff_label, points_range) = match difficulty.unwrap_or("medium") {
        "easy" => ("Easy", "10-30"),
        "hard" => ("Hard", "60-100"),
        _ => ("Medium", "30-60"),
    };

    let count_str = if count == 1 {
        "Generate 1 mission".to_string()
    } else {
        format!("Generate {count} unique, engaging missions")
    };

    format!(
        r#"You are a creative mission generator for "Complice", an outdoor activity and self-improvement app. {count_str} based on the user's context.

User Profile:
- Interests: {interests_str}
- Weather: {weather}
- Time of day: {time_of_day}
- Difficulty: {diff_label}
{mood_line}
- {recent_str}

Mission Requirements:
- Each mission must be realistically doable within the given time limit and radius
- Missions should be varied and encourage exploration, creativity, or self-improvement
- Points should be in range {points_range} (difficulty: {diff_label})
- Descriptions should be inspiring and actionable (1-2 sentences)
- Avoid repeating or being too similar to recently completed missions

Respond with ONLY valid JSON (no markdown, no extra text):
{{"missions":[{{"title":"Mission Title","description":"What to do.","time_limit_minutes":30,"radius_meters":500,"points":50}},...]}}"#,
        mood_line = if mood_str.is_empty() {
            String::new()
        } else {
            format!("- {mood_str}")
        }
    )
}
