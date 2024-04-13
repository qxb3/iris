use std::process;
use clap::{arg, error::ErrorKind, value_parser, Command};

mod client;
mod server;
mod command;

#[tokio::main]
async fn main() {
    let mut command = command();

    match command.clone().get_matches().subcommand() {
        Some(("server", sub)) => {
            let port = sub.get_one::<u32>("port").unwrap();
            let format = match sub.get_one::<String>("format").unwrap() {
                f if f == "default" || f == "json" => f.to_string(),
                _ => {
                    let error = command.error(ErrorKind::InvalidValue, "Invalid format value.\nvalid values: ('default', 'json')");
                    println!("{error}");

                    process::exit(1);
                }
            };
            let debug = sub.get_one::<bool>("debug").unwrap();

            server::start(format!("127.0.0.1:{port}").as_str(), format, debug.to_owned()).await;
        }
        Some(("client", sub)) => {
            let host = sub.get_one::<String>("host").unwrap();
            let port = sub.get_one::<u32>("port").unwrap();

            client::start(format!("{host}:{port}").as_str()).await;
        }
        _ => unreachable!(),
    }
}

fn command() -> Command {
    Command::new("iris")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand_required(true)
        .subcommand(
            Command::new("server")
                .about("Start the in-memory server")
                .args(vec![
                    arg!(-p --port <number> "The port the server will run on")
                        .value_parser(value_parser!(u32))
                        .default_value("3000")
                        .required(false),
                    arg!(-f --format <string> "The format of the server response ('default', 'json')")
                        .value_name("json")
                        .value_parser(value_parser!(String))
                        .default_value("default")
                        .required(false),
                    arg!(-d --debug "The port the server will run on")
                        .value_parser(value_parser!(bool))
                        .default_value("false")
                        .required(false),
                ]),
        )
        .subcommand(
            Command::new("client")
                .about("Enter client repl mode")
                .args(vec![
                    arg!(--host <string> "The url the client will connect to")
                        .value_parser(value_parser!(String))
                        .default_value("127.0.0.1")
                        .required(false),
                    arg!(-p --port <number> "The port the client connect to")
                        .value_parser(value_parser!(u32))
                        .default_value("3000")
                        .required(false),
                ]),
        )
}
