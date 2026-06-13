// ZADANIE: Napisz async funkcję fetch_weather(lat: f64, lon: f64).
//
// Powinna:
// 1. Zbudować URL do Open-Meteo:
//    https://api.open-meteo.com/v1/forecast
//    z parametrami: latitude, longitude, current=temperature_2m,wind_speed_10m,weather_code
//
// 2. Wywołać reqwest::get(url).await?
// 3. Deserializować odpowiedź przez .json::<WeatherResponse>().await?
// 4. Zwrócić Result<WeatherResponse, reqwest::Error>
//
// Pamiętaj: async fn musi mieć .await tam gdzie czekasz na I/O.
// Znak ? propaguje błąd w górę — funkcja musi zwracać Result<_, E>.

use crate::models::WeatherResponse;

pub async fn fetch_weather(lat: f64, lon: f64) -> Result<WeatherResponse, reqwest::Error>{
    let url_meteo = format!("https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,wind_speed_10m,weather_code", lat, lon);
    let result = reqwest::get(url_meteo).await?;
    let json_respone = result.json::<WeatherResponse>().await?;
    Ok(json_respone)

}
