use clap::{Parser, Subcommand};
mod api;
mod types;
mod env;
mod cli;
mod r#impl;

#[derive(Parser)]
#[command(name = "weather", about = "A simple weather CLI", version = "1.0")]
struct Cli {
    /// The ISO 3166 zipcode to get weather information.
    #[arg(required = false)]
    zipcode: Option<String>,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let env = env::env().expect("ENV variable expected");
    dotenv::from_filename(format!("{env}.env"))
        .expect(format!("{env}.env file not found").as_str());

    match (&cli.zipcode, &cli.command) {
        (Some(zipcode), None) => {
            cli::get_weather_for(zipcode).await;
        },
        _ => eprintln!("No valid command or zipcode provided"),
    }
}
