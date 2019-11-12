use crate::models::forecast::ForecastResponse;
use crate::models::weather::WeatherResponse;
use crate::result::Result;
use dotenv;
pub fn fetch_current_weather(city: String) -> Result<WeatherResponse> {
    fetch_weather(city, "weather".to_owned())
}

pub fn fetch_forecast(city: String) -> Result<ForecastResponse> {
    fetch_weather(city, "forecast".to_owned())
}

fn fetch_weather<T>(city: String, weather_type: String) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let request_url = format!(
        "
    https://api.openweathermap.org/data/2.5/{weather_type}?q={city},uk&units=metric&appId={api_key}",
        weather_type = weather_type,
        api_key = dotenv::var("API_KEY").unwrap(),
        city = city
    );
    let mut response = reqwest::get(&request_url)?;
    let weather_response: T = response.json()?;
    Ok(weather_response)
}
