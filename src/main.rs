use serde_json::Value;
use dotenv::dotenv;
use std::{env, error::Error};
use reqwest;
use tokio;

fn print_weather_output(body: String) -> Result<(), serde_json::Error> {
    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(&body)?;

    // Access parts of the data by indexing with square brackets.
    let name = v["name"].as_str().unwrap_or("unknown");
    let weather = v["weather"][0]["main"].as_str().unwrap_or("unknown");
    let temperature = v["main"]["temp"].as_f64().unwrap_or_default();

    println!("Weather in [{}] is [{}]. Temperature is [{:.1}°C]", name, weather, temperature);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    dotenv().ok();
    // The URL of the API you want to call
    let api_key = env::var("OPENWEATHER_KEY").expect("Key not found"); // Replace with your actual API key
    let city = "Regensburg";
    let url = format!("http://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric", city, api_key);

    // Send a GET request and await the response
    let response = reqwest::get(url).await?;

    // Check if the request was successful
    if response.status().is_success() {
        // Parse the response text and print it
        let body = response.text().await?;
        //println!("Response Text: {}", body); debug reasons
        print_weather_output(body);
    } else {
        // Handle the error case
        println!("Request failed with status: {}", response.status());
    }
    Ok(())
}