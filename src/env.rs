pub fn geocoder_api_url() -> Option<String> {
    std::env::var("GEOCODER_API_URL").ok()
}

pub fn weather_api_url() -> Option<String> {
    std::env::var("WEATHER_API_URL").ok()
}

pub fn weather_api_key() -> Option<String> {
    std::env::var("OPEN_WEATHER_API_KEY").ok()
}

// pub fn mongodb_uri() -> Option<String> {
//     std::env::var("MONGO_DB_CONNECTION_STRING").ok()
// }

pub fn env() -> Option<String> {
    std::env::var("ENV")
        .ok()
        .map(|x| x.to_lowercase())
        .or_else(|| Some("".to_string()))
}
