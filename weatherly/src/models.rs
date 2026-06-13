use serde::Deserialize;

#[derive(Deserialize)]
pub struct WeatherResponse {
    pub current: CurrentWeather,
    pub current_units: CurrentUnits,
}

#[derive(Deserialize)]
pub struct CurrentWeather {
    pub temperature_2m: f64,
    pub wind_speed_10m: f64,
    pub weather_code: i32,
}

#[derive(Deserialize)]
pub struct CurrentUnits {
    pub temperature_2m: String,
    pub wind_speed_10m: String,
}
