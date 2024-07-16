use crate::types::{GeoLocation, WeatherResponse};
use prettytable::{format, row, Table};
impl GeoLocation {
    pub fn display(&self) {
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

        // Add headers
        table.set_titles(row![FCc => "Latitude(°)", "Longitude(°)"]);

        // Add GeoLocation data
        table.add_row(row![format!("{}", self.lat), format!("{}", self.lon),]);

        // Print the table to stdout
        table.printstd();
    }
}

impl WeatherResponse {
    pub fn display(&self) {
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

        // Add headers
        table.set_titles(row![FCc => "Temp(°C)", "Feels Like(°C)", "Min Temp(°C)", "Max Temp(°C)", "Pressure(hPa)", "Humidity(%)", "Weather"]);

        // Add weather data
        table.add_row(row![
            format!("{:.2}", self.main.temp - 273.0),
            format!("{:.2}", self.main.feels_like - 273.0),
            format!("{:.2}", self.main.temp_min - 273.0),
            format!("{:.2}", self.main.temp_max - 273.0),
            self.main.pressure.to_string(),
            self.main.humidity.to_string(),
            format!("{}-{}", self.weather[0].main, self.weather[0].description)
        ]);

        // Print the table to stdout
        table.printstd();
    }
}
