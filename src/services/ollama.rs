use reqwest::Client;

use crate::error::ApiError;
use crate::models::{
    OllamaGenerateRequest, OllamaGenerateResponse, OllamaMissionsResponse,
    OllamaTagsResponse, MissionData,
};

pub async fn get_models(ollama_url: &str) -> Result<Vec<String>, ApiError> {
    let client = Client::new();
    let url = format!("{ollama_url}/api/tags");

    let resp: OllamaTagsResponse = client
        .get(&url)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    let models: Vec<String> = resp
        .models
        .into_iter()
        .map(|m| {
            m.name
                .split(':')
                .next()
                .unwrap_or(&m.name)
                .to_string()
        })
        .collect();

    Ok(models)
}

pub async fn generate_missions(
    ollama_url: &str,
    model: &str,
    prompt: &str,
) -> Result<Vec<MissionData>, ApiError> {
    let client = Client::new();
    let url = format!("{ollama_url}/api/generate");

    let body = OllamaGenerateRequest {
        model: model.to_string(),
        prompt: prompt.to_string(),
        stream: false,
        format: "json".to_string(),
    };

    let resp: OllamaGenerateResponse = client
        .post(&url)
        .json(&body)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    let parsed: OllamaMissionsResponse = serde_json::from_str(&resp.response)?;

    if parsed.missions.is_empty() {
        return Err(ApiError::Internal("Ollama returned empty missions".into()));
    }

    Ok(parsed.missions.into_iter().take(3).collect())
}

pub async fn check_health(ollama_url: &str) -> Result<bool, ApiError> {
    let client = Client::new();
    let url = format!("{ollama_url}/api/tags");

    match client.get(&url).send().await {
        Ok(resp) => Ok(resp.status().is_success()),
        Err(_) => Ok(false),
    }
}
