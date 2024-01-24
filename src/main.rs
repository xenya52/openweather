use dotenv::dotenv;
use std::{env, error::Error};
use reqwest;
use tokio;

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
        println!("Response Text: {}", body);
    } else {
        // Handle the error case
        println!("Request failed with status: {}", response.status());
    }
    Ok(())
}