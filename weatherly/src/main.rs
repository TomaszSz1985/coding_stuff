// ZADANIE: Połącz wszystko razem.
//
// 1. Dodaj atrybut #[tokio::main] nad async fn main()
// 2. Zawołaj api::fetch_weather(52.2297, 21.0122).await  <- Warszawa
// 3. Obsłuż Result (unwrap() lub match, albo propaguj przez ?)
// 4. Wywołaj display::print_weather(&response)
//
// KLUCZOWA NAUKA:
//   #[tokio::main]  — makro które zamienia main() w blokujący executor
//   async fn main() — main musi być async żeby móc używać .await
//   .await          — tu program "czeka" na wynik operacji I/O bez blokowania wątku

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
