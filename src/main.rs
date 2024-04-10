use clap::{arg, value_parser, Command};

mod client;
mod server;
mod core;

#[tokio::main]
async fn main() {
    let matches = command().get_matches();

    match matches.subcommand() {
        Some(("server", sub)) => {
            let port = sub.get_one::<u32>("port").unwrap();
            let debug = sub.get_one::<bool>("debug").unwrap();

            server::start(format!("127.0.0.1:{port}").as_str(), debug.to_owned()).await;
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
                    arg!(-d --debug "The port the server will run on")
                        .value_parser(value_parser!(bool))
                        .default_value("false")
                        .required(false),
                ])
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
                        .required(false)
                ])
        )
}
