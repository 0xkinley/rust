use reqwest::Error;
use serde::Deserialize;
use std::env;
use dotenv::dotenv;

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    main: Main,
    weather: Vec<Weather>,
    name: String,
}

#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    humidity: u8,
}

#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    
    dotenv().ok();
    
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: weather_cli <CITY_NAME>");
        return Ok(());
    }
    let city = &args[1];
    

    let api_key = env::var("OPENWEATHER_API_KEY").expect("API key not found in .env file");

    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city, api_key
    );

    let response = reqwest::get(&url).await?.json::<WeatherResponse>().await?;

    println!("Weather in {}:", response.name);
    println!("Temperature: {}°C", response.main.temp);
    println!("Humidity: {}%", response.main.humidity);
    println!("Condition: {}", response.weather[0].description);

    Ok(())
}
