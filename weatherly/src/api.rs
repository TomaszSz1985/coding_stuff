
use crate::models::WeatherResponse;

pub async fn fetch_weather(lat: f64, lon: f64) -> Result<WeatherResponse, reqwest::Error>{
    let url_meteo = format!("https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,wind_speed_10m,weather_code", lat, lon);
    let result = reqwest::get(url_meteo).await?;
    let json_respone = result.json::<WeatherResponse>().await?;
    Ok(json_respone)

}
