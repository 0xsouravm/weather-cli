use crate::api::{get_lat_long, get_weather};
pub async fn get_weather_for(zipcode: &str) {
    let geo_location = get_lat_long(zipcode).await;
    if geo_location.is_err() {
        eprintln!("Error getting lat/long: {:?}", geo_location.err());
        return;
    }

    let geo_location = geo_location.unwrap();
    println!("Coordinates:");
    geo_location.display();

    let weather = get_weather(geo_location.lat.to_string(), geo_location.lon.to_string()).await;
    if weather.is_err() {
        eprintln!("Error getting weather: {:?}", weather.err());
        return;
    }
    println!("Weather for {}:", geo_location.name);
    weather.unwrap().display();
}

// Login and Logout functions can be implemented here
