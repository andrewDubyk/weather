mod api;
mod configuration;

use clap::{Arg, Command};

fn main() -> Result<(), String> {
    let matches = Command::new(clap::crate_name!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("configure")
                .about("Configure credentials for <provider>, where <provider> is some short name for a concrete weather API")
                .arg(
                    Arg::new("provider")
                        .help("Set provider name")
                        .required(true)
                        .possible_values(["openweather", "weatherapi"])
                )
                .arg(
                    Arg::new("api_key")
                        .help("Set provider API key")
                )
                .arg_required_else_help(true)
        )
        .subcommand(
            Command::new("get")
                .about("Show weather for the provided <address>")
                .arg(
                    Arg::new("address")
                        .help("Address where to get weather information")
                        .required(true)
                )
                .arg(
                    Arg::new("date")
                        .help("Show weather for the address at given date")
                        .required(false)
                )
                .arg_required_else_help(true),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("configure", subcommand_matches)) => {
            let provider = subcommand_matches.value_of("provider").unwrap();
            let api_key = subcommand_matches.value_of("api_key");
            match configuration::configure_provider(provider, api_key) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }
        Some(("get", subcommand_matches)) => {
            let address = subcommand_matches.value_of("address").unwrap();
            let date = subcommand_matches.value_of("date");
            match configuration::get_provider_info() {
                Ok(config) => {
                    let provider = api::new_provider(config.0, config.1);
                    match provider.get_weather(address, date) {
                        Ok(res) => {
                            println!("Result : {}", res)
                        }
                        Err(e) => return Err(e),
                    }
                }
                Err(e) => return Err(e),
            }
        }
        _ => {
            eprintln!("Wrong number of arguments provided");
            std::process::exit(exitcode::USAGE);
        }
    };

    Ok(())
}
