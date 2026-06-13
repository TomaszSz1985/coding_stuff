// ZADANIE: Zdefiniuj struktury danych odpowiadające odpowiedzi API Open-Meteo.
//
// API zwraca JSON w formacie:
// {
//   "current": {
//     "temperature_2m": 18.5,
//     "wind_speed_10m": 12.3,
//     "weather_code": 3
//   },
//   "current_units": {
//     "temperature_2m": "°C",
//     "wind_speed_10m": "km/h"
//   }
// }
//
// Potrzebujesz 3 struktury: WeatherResponse, CurrentWeather, CurrentUnits.
// Każda musi mieć atrybut: #[derive(Deserialize)]
// oraz "use serde::Deserialize;" na górze pliku.
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
