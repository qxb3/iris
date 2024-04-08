use clap::{command, Command, Parser};

mod server;
mod client;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {

}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("server", _sub_matches)) => {
            server::start("127.0.0.1:3000");
        },
        Some(("client", _sub_matches)) => {
            client::start("127.0.0.1:3000");
        },
        _ => unreachable!()
    }

}

fn cli() -> Command {
    Command::new("iris")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand_required(true)
        .subcommand(
            Command::new("server")
                .about("Start the in-memory server")
        )
        .subcommand(
            Command::new("client")
                .about("Enter client repl mode")
        )
}
