mod display;
mod file;
mod result;
mod models;
mod networking;

use clap::{App, Arg, ArgMatches, SubCommand};

use display::print::print_forecast_info;
use file::write::{read_file, write_file};

use networking::network::{fetch_current_weather, fetch_forecast};
use result::{error, Result};
fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("location")
                .help("The location to find the weather for")
                .short("l")
                .long("location")
                .takes_value(true),
        )
        .subcommand(
            SubCommand::with_name("current")
                .about("The current weather")
                .arg(Arg::with_name("current").short("c")),
        )
        .subcommand(
            SubCommand::with_name("forecast")
                .about("The 5 day weather forecast")
                .arg(Arg::with_name("forecast").short("f")),
        )
        .get_matches();

    extract_city_arg(&matches).and_then(|city| handle_subcommand(city, &matches));
    read_file();
}

fn extract_city_arg(matches: &ArgMatches) -> Result<String> {
    let city = matches
        .value_of("location")
        .ok_or(error::WeatherError::ParseError(
            "Failed to pass a location".to_owned(),
        ))?;
    Ok(city.to_owned())
}

fn handle_subcommand(city: String, matches: &ArgMatches) -> Result<()> {
    match matches.subcommand() {
        ("current", Some(_matches)) => {
            println!("The city {:?}", &city);
            fetch_current_weather(city).and_then(|weather_response| {
                println!("The weather is {:#?}", &weather_response);
                write_file(weather_response)
            })?;
            Ok(())
        }
        ("forecast", Some(_matches)) => {
            let fetch_result =
                fetch_forecast(city).expect("The weather forecast failed to request");
            print_forecast_info(fetch_result);
            Ok(())
        }
        _ => {
            println!("You tried to search for {:?} but didn't specify what type of information you wanted", &city);
            Ok(())
        }
    }
}
