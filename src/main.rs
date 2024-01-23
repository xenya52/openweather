use reqwest;
use dotenv::dotenv;
use serde::Deserialize;
use std::{env, error::Error};

#[derive(Deserialize)]
struct WeatherResponse {
    main: Main,
    // Add other fields as needed
}

#[derive(Deserialize)]
struct Main {
    temp: f32,
    // Add other fields as needed
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let api_key = env::var("OPENWEATHER_KEY").expect("Key not found"); // Replace with your actual API key
    let city = "Regensburg";
    let url = format!("http://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric", city, api_key);

    let response = reqwest::get(&url).await?.json::<WeatherResponse>().await?;

    println!("Current temperature in {} is {}°C", city, response.main.temp);

    Ok(())
}