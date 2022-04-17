use clap::{Arg, Command};
use log::error;

fn main() {
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
                )
                .arg(
                    Arg::new("api_key")
                        .help("Set provider API key")
                        .required(true)
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
            let api_key = subcommand_matches.value_of("api_key").unwrap();
            println!("provider ({}) api_key({})", provider, api_key);
        }
        Some(("get", subcommand_matches)) => {
            let address = subcommand_matches.value_of("address").unwrap();
            let date = subcommand_matches.value_of("date").unwrap_or_default();
            println!("address ({}) date({})", address, date);
        }
        _ => error!("Wrong number of arguments provided"),
    }
}
