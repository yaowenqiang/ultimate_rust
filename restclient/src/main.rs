use anyhow;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Weather {
    latitude: f64,
    longitude: f64,
    current_weather: CurrentWeather,
}

#[derive(Deserialize, Debug)]
struct CurrentWeather {
    temperature: f64,
    windspeed: f64,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    const URL: &str = "http://localhost:8080";
    let response = reqwest::get(URL).await?;
    // println!("{:?}", response.text().await?);
    println!("{:?}", response.json().await?);
    Ok(())
}
