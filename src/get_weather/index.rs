use reqwest::Error;

pub async fn get_weather(key: Option<String>, city: Option<String>) -> Result<String, Error> {
    let url: String = format!(
        "https://restapi.amap.com/v3/weather/weatherInfo?key={}&city={}",
        key.unwrap(), city.unwrap(),
    );

    let body = reqwest::get(url)
        .await?
        .text()
        .await?;
    return Ok(body);
}