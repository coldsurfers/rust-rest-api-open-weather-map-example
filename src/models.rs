use serde::Deserialize;

#[derive(Deserialize)]
pub struct WeatherData {
    pub main: Main,
    pub weather: Vec<Weather>
}

#[derive(Deserialize)]
pub struct Main {
    pub temp: f64
}

#[derive(Deserialize)]
pub struct Weather {
    pub description: String
}