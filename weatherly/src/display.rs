// ZADANIE: Napisz funkcję print_weather(response: &WeatherResponse).
//
// Powinna wyświetlić dane jako tabelkę używając crate'u `tabled`.
// Kluczowe elementy:
//   - zbuduj Vec<Row> gdzie Row to struct z polami: parametr, wartość, jednostka
//   - Row musi mieć #[derive(Tabled)]
//   - użyj tabled::Table::new(&rows).to_string() żeby wydrukować tabelkę
//
// Przykład wyjścia:
// +------------------+-------+-------+
// | Parametr         | Wartość | Jednostka |
// +------------------+-------+-------+
// | Temperatura      | 18.5  | °C    |
// | Prędkość wiatru  | 12.3  | km/h  |
// | Kod pogody       | 3     |       |
// +------------------+-------+-------+

use crate::models::WeatherResponse;
use tabled::{Table, Tabled};

#[derive(Tabled)]
struct Row {
    parameter: String,
    value: String,
    unit: String,
}

fn weather_code_description(code: i32) -> &'static str {
    match code {
        0 => "Clear sky",
        1..=3 => "Cloudy",
        45..=48 => "Fog",
        51..=55 => "Drizzle (light → heavy)",
        61..=65 => "Rain (light → heavy)",
        71..=75 => "Snow (light → heavy)",
        80..=82 => "Rain showers",
        95 => "Thunderstorm",
        96..=99 => "Thunderstorm with hail",
        _ => "Other"
    }
}

pub fn print_weather(response: &WeatherResponse) {
    let mut rows: Vec<Row> = Vec::new();
    rows.push(Row {
        parameter: "Temperature".to_string(),
        value: response.current.temperature_2m.to_string(),
        unit: response.current_units.temperature_2m.to_string(),
    });
    rows.push(Row {
        parameter: "Wind speed".to_string(),
        value: response.current.wind_speed_10m.to_string(),
        unit: response.current_units.wind_speed_10m.to_string(),
    });
    rows.push(Row {
        parameter: "Weather code".to_string(),
        value: response.current.weather_code.to_string(),
        unit: weather_code_description(response.current.weather_code).to_string(),
    });
    let table = Table::new(&rows).to_string();
    println!("{}", table);
}
