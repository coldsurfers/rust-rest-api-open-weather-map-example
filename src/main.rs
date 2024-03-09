mod models;
mod fetchers;

#[tokio::main]
async fn main() {
    use crate::fetchers::fetch_weather;
    let api_key = "YOUR_API_KEY";
    let mut city = String::new();
    println!("Enter the city name:");
    std::io::stdin().read_line(
        &mut city
    ).expect("Failed to read line");
    let city_clone = city.clone();
    tokio::spawn(fetch_weather(api_key, city_clone));
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
}
