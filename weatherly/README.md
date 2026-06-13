# weatherly

A CLI weather app written in Rust as a learning project.

Fetches current weather data from the [Open-Meteo](https://open-meteo.com/) free API (no API key required) and displays it as a formatted table.

## Usage

```
cargo run
```

Displays current temperature, wind speed, and weather code for Warsaw, Poland.

## Example output

```
+-----------------+---------+----------+
| parameter       | value   | unit     |
+-----------------+---------+----------+
| Temperature     | 14      | °C       |
+-----------------+---------+----------+
| Wind speed      | 8.3     | km/h     |
+-----------------+---------+----------+
| Weather code    | 51      |          |
+-----------------+---------+----------+
```

## What I learned

- `async fn` and `.await` — non-blocking I/O model in Rust
- `#[tokio::main]` — macro that sets up the async runtime for `main()`
- `reqwest` — making async HTTP GET requests
- `serde::Deserialize` — deriving JSON deserialization for structs automatically
- `tabled::Tabled` — rendering a `Vec<struct>` as a formatted terminal table
- `Box<dyn std::error::Error>` — ergonomic error propagation with `?` in `main()`

## Dependencies

| Crate    | Purpose                  |
|----------|--------------------------|
| tokio    | async runtime            |
| reqwest  | HTTP client              |
| serde    | JSON deserialization      |
| tabled   | terminal table rendering |

## Project structure

```
src/
├── main.rs      — entry point, #[tokio::main]
├── models.rs    — serde structs matching the API response
├── api.rs       — async HTTP fetch function
└── display.rs   — table rendering
```
