use reqwest::blocking::get;
use serde::Deserialize;
use std::env;
use dotenv::dotenv;

#[derive(Debug, Deserialize)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
}

#[derive(Debug, Deserialize)]
struct Weather {
    description: String,
}

#[derive(Debug, Deserialize)]
struct Main {
    temp: f64,
}

fn main() {
    dotenv().ok();
    let api_key = env::var("OPENWEATHER_API_KEY").expect("API Key not found");
    let city = "Tokyo";
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city, api_key
    );

    let res = get(&url).expect("Request failed");
    let weather: WeatherResponse = res.json().expect("Failed to parse JSON");

    println!("現在の{}の天気: {}", city, weather.weather[0].description);
    println!("気温: {}°C", weather.main.temp);
}
