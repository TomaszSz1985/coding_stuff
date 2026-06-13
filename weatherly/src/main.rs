mod api;
mod display;
mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
println!("Location: Gnaszyn 52.2297, 21.0122");
let result = api::fetch_weather(50.8118, 19.1203).await?;
display::print_weather(&result);
Ok(())
}
