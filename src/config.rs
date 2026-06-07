use std::env;

pub struct Config {
    pub port: u16,
    pub database_url: String,
    pub ollama_url: String,
    pub default_model: String,
    pub weather_api_endpoint: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".into())
                .parse()
                .expect("PORT must be a valid u16"),
            database_url: env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set"),
            ollama_url: env::var("OLLAMA_URL")
                .unwrap_or_else(|_| "http://localhost:11434".into()),
            default_model: env::var("DEFAULT_MODEL")
                .unwrap_or_else(|_| "llama3".into()),
            weather_api_endpoint: env::var("WEATHER_API_ENDPOINT")
                .unwrap_or_else(|_| "https://api.open-meteo.com/v1/forecast".into()),
        }
    }
}
