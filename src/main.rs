mod get_weather;
mod read_config;
mod write_data;
use crate::get_weather::index::get_weather;
use crate::read_config::index::read_config;
use crate::write_data::index::write_data;

use json::JsonValue;
use json::JsonError;

#[tokio::main]
async fn main() {
    let mut api_key: Option<String> = None;
    let mut city: Option<String> = None;
    read_config(&mut api_key, &mut city);

    if api_key == None {
        panic!("Wrong key to API.");
    }

    let weather_res =
        get_weather(api_key, city).await;
    let weather_data: Result<JsonValue, JsonError>;
    match weather_res {
        Ok(s) => {
            // let weather_data_str = Some(s.clone());
            weather_data = json::parse(&s);
        },
        Err(_e) => {
            panic!("Get weather error.");
        }
    }
    match weather_data {
        Ok(data) => {
            let status = data["status"].clone();
            if status == "1" {
                let live_val = data["lives"][0].clone();
                let weather = live_val["weather"].clone();
                let temperature = live_val["temperature"].clone();

                println!("Weather: {}", weather);
                println!("Temperature: {}", temperature);

                #[warn(unused_must_use)]
                write_data(String::from("./weather.txt"), weather.to_string());
                write_data(String::from("./temperature.txt"), temperature.to_string());
            }
        },
        Err(_e) => {},
    }
}