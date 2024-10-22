# Weather CLI

A simple command-line interface (CLI) application for retrieving weather information.

This project serves as a demonstration for the clap and reqwest crates.

## Features

- Get weather information for a specific zipcode

## Installation

To install the Weather CLI, make sure you have Rust and Cargo installed on your system. Then, clone the repository and build the project:

```bash
git clone https://github.com/0xsouravm/weather-cli.git
cd weather-cli

cargo build --release
cargo install --path .
export PATH="$HOME/.cargo/bin:$PATH"
```

## Usage
### Get Weather Information
To get weather information for a specific zipcode:

```bash
weather <zipcode>
```
Replace <zipcode> with the ISO 3166 zipcode for which you want to retrieve weather information. Ex - E14,GB (London)

## Configuration
The application uses environment variables for configuration. Create a .env file in the project root directory with the following variables:
```text
GEOCODER_API_URL="http://api.openweathermap.org/geo/1.0/zip?zip={zip}&appid={api_key}"
WEATHER_API_URL="https://api.openweathermap.org/data/2.5/weather?lat={lat}&lon={long}&appid={api_key}"
OPEN_WEATHER_API_KEY=<your_api_key>
```

Replace the placeholder with your actual API key.
## Project Structure
- ```main.rs```: Contains the main application logic and CLI structure
- ```types.rs```: Defines the types used in the application
- ```api.rs```: Handles API requests to the weather and geocoding services
- ```env.rs```: Exposes functions to fetch environment variables
- ```cli.rs```: Implements CLI-specific functionality
- ```impl.rs```: Contains additional implementation details to display the data
## Dependencies
- [clap](https://crates.io/crates/clap): For parsing command-line arguments
- [reqwest](https://crates.io/crates/reqwest): For making HTTP requests
- [serde](https://crates.io/crates/serde): For serializing and deserializing JSON data
- [tokio](https://crates.io/crates/tokio): For asynchronous runtime
- [dotenv](https://crates.io/crates/dotenv): For loading environment variables from a file
