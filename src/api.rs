use crate::env::*;
use crate::types::*;
use reqwest::Error;

pub async fn get_lat_long(zip_code: &str) -> Result<GeoLocation, Error> {
    let url = geocoder_api_url().expect("NO_GEOCODER_API_URL");
    let api_key = weather_api_key().expect("NO_OPEN_WEATHER_API_KEY");

    let url = url
        .replace("{zip}", &zip_code)
        .replace("{api_key}", &api_key);

    let response = reqwest::get(&url).await?.json::<GeoLocation>().await?;

    Ok(response)
}

pub async fn get_weather(lat: String, long: String) -> Result<WeatherResponse, Error> {
    let url = weather_api_url().expect("NO_WEATHER_API_URL");
    let api_key = weather_api_key().expect("NO_OPEN_WEATHER_API_KEY");

    let url = url
        .replace("{lat}", &lat)
        .replace("{long}", &long)
        .replace("{api_key}", &api_key);

    let response = reqwest::get(&url).await?.json::<WeatherResponse>().await?;

    Ok(response)
}
