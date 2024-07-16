use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GeoLocation {
    pub lat: f64,
    pub lon: f64,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct WeatherResponse {
    pub main: MainWeather,
    pub weather: Vec<Weather>,
}

#[derive(Deserialize, Debug)]
pub struct MainWeather {
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub pressure: u32,
    pub humidity: u32,
}

#[derive(Deserialize, Debug)]
pub struct Weather {
    pub id: u32,
    pub main: String,
    pub description: String,
    pub icon: String,
}
