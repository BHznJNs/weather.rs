mod get_weather;
mod read_config;
mod write_data;

use crate::get_weather::index::get_weather;
use crate::read_config::index::read_config;
use crate::read_config::get_base::get_base;
use crate::write_data::index::write_data;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut api_key: Option<String> = None;
    let mut city: Option<String> = None;
    read_config(&mut api_key, &mut city)?;

    if api_key == None {
        panic!("Wrong key to API.");
    }

    let weather_str =
        get_weather(api_key, city).await?;
    let weather_data = json::parse(&weather_str)?;
    
    // start analyse data
    let status = weather_data["status"].clone();
    if status == "1" {
        let live_val = weather_data["lives"][0].clone();
        let weather = live_val["weather"].clone();
        let temperature = live_val["temperature"].clone();

        println!("Weather: {}", weather);
        println!("Temperature: {}", temperature);

        let weather_path = get_base("./weather.txt")?;
        let temperature_path = get_base("./temperature.txt")?;

        write_data(weather_path, weather.to_string())?;
        write_data(temperature_path, temperature.to_string())?;
    }
    Ok(())
}