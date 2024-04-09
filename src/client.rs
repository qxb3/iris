use std::{io::{BufRead, BufReader, Write}, net::TcpStream, process};
use indoc::indoc;

macro_rules! write_error {
    ($message:expr) => {
        {
            println!("{}\n", $message);
            continue;
        }
    };
}

macro_rules! handle_reply {
    ($stream:expr) => {
        {
            let mut buf_reader = BufReader::new(&mut $stream);

            let mut buffer = String::new();
            match buf_reader.read_line(&mut buffer) {
                Ok(byte) => {
                    if byte == 0 {
                        write_error!("Connection closed");
                    }

                    println!("{buffer}");
                },
                Err(err) => write_error!(format!("Failed: {err}"))
            }
        }
    };
}

pub fn start(addr: &str) {
    match TcpStream::connect(addr) {
        Ok(mut stream) => {
            let local_addr = &stream.peer_addr().unwrap();
            println!(
                indoc! {"

                      ▀             ▀
                    ▄▄▄     ▄ ▄▄  ▄▄▄     ▄▄▄
                      █     █▀  ▀   █    █   ▀
                      █     █       █     ▀▀▀▄
                    ▄▄█▄▄   █     ▄▄█▄▄  ▀▄▄▄▀

                    Client is connected.
                    • version:  {}
                    • host:     http://{}
                    • port:     {}
                "},
                env!("CARGO_PKG_VERSION"),
                local_addr,
                local_addr.port()
            );

            loop {
                if let Ok(line) = readline() {
                    if line.is_empty() {
                        continue;
                    }

                    let mut parts = line.split(' ')
                        .filter(|part| !part.is_empty())
                        .map(|part| part.trim());

                    let command = match parts.next() {
                        Some(command) => command.to_uppercase(),
                        None => write_error!("No command specified")
                    };

                    let id = match parts.next() {
                        Some(id) => match id.parse::<u32>() {
                            Ok(id) => id,
                            Err(_) => write_error!("ID needs to be a number")
                        },
                        None => write_error!("No ID specified. \"<command> <id> [data]\"")
                    };

                    let data: Option<String> = match parts.next() {
                        Some(data) => Some(data.to_string()),
                        None => {
                            if command == "SET" || command == "UPD" {
                                write_error!("<data> is required for \"{command}\". <command> <id> <data>")
                            } else {
                                None
                            }
                        }
                    };

                    match command.as_str() {
                        "GET" | "DEL" => {
                            match &stream.write_all(format!("{command} {id}\n").as_bytes()) {
                                Ok(_) => handle_reply!(stream),
                                Err(err) => write_error!(format!("Failed: {err}"))
                            }
                        },
                        "SET" | "UPD" => {
                            match stream.write_all(format!("{command} {id} {}\n", data.unwrap()).as_bytes()) {
                                Ok(_) => handle_reply!(stream),
                                Err(err) => write_error!(format!("Failed: {err}"))
                            }
                        },
                        _ => println!("err Invalid command")
                    }
                }
            }
        }
        Err(err) => {
            println!("Failed to connect: {err}");
            process::exit(1);
        },
    }
}

fn readline() -> Result<String, String> {
    write!(std::io::stdout(), "iris@{} $ ", env!("CARGO_PKG_VERSION")).map_err(|e| e.to_string())?;
    std::io::stdout().flush().map_err(|e| e.to_string())?;

    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .map_err(|e| e.to_string())?;

    Ok(buffer.trim().to_string())
}
