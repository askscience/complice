use reqwest::Client;

use crate::error::ApiError;
use crate::models::{Location, OpenMeteoResponse};

pub async fn get_weather(
    endpoint: &str,
    location: &Location,
) -> Result<String, ApiError> {
    let client = Client::new();
    let url = format!(
        "{}?latitude={}&longitude={}&current_weather=true",
        endpoint, location.lat, location.lon
    );

    let resp: OpenMeteoResponse = client
        .get(&url)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    match resp.current_weather {
        Some(w) => {
            let desc = weather_code_description(w.weathercode);
            Ok(format!(
                "{:.0}°C, {} (wind {:.0} km/h)",
                w.temperature, desc, w.windspeed
            ))
        }
        None => Ok("Weather data unavailable".into()),
    }
}

fn weather_code_description(code: i32) -> &'static str {
    match code {
        0 => "clear sky",
        1 => "mainly clear",
        2 => "partly cloudy",
        3 => "overcast",
        45 | 48 => "foggy",
        51 => "light drizzle",
        53 => "moderate drizzle",
        55 => "dense drizzle",
        61 => "slight rain",
        63 => "moderate rain",
        65 => "heavy rain",
        71 => "slight snowfall",
        73 => "moderate snowfall",
        75 => "heavy snowfall",
        80 => "slight rain showers",
        81 => "moderate rain showers",
        82 => "violent rain showers",
        95 => "thunderstorm",
        96 | 99 => "thunderstorm with hail",
        _ => "unknown conditions",
    }
}
