use reqwest;
use super::models::WeatherData;

pub async fn fetch_weather(api_key: &str, city: String) -> Result<(), reqwest::Error> {
    // Build the URL for the OpenWeatherMap API request
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}",
        city.trim(),
        api_key
    );
// Make the API request and handle errors
    let response = reqwest::get(&url).await?;
    // Check if the request was successful (status code 200)
    if response.status().is_success() {
        // Parse the JSON response into our WeatherData struct
        let weather_data: WeatherData = response.json().await?;
        // Extract and print relevant weather information
        let temperature = weather_data.main.temp;
        let description = &weather_data.weather[0].description;
        println!("Weather in {}: {:.2}°C, {}", city.trim(), temperature, description);
    } else {
        // Print an error message if the request was not successful
        println!("Error: {}", response.status());
    }
    Ok(())
}