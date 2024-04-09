use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
    process,
};

use indoc::{indoc, printdoc};

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
                        println!("Connection closed.");
                        process::exit(1);
                    }

                    println!("> {buffer}");
                },
                Err(err) => write_error!(format!("> Failed: {err}"))
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

                    let command = line.clone().splitn(3, ' ').collect::<Vec<&str>>().get(0).unwrap().to_lowercase();
                    match command.as_str() {
                        "help" => printdoc! {"
                            • What is iris?
                              iris is a simple key value database,
                              every value in iris is considered to be a string (for now)
                              and you, yourself will be the one to parse the types.

                            • Commands
                              - SET <id:number> <data:string>: sets a value on a key.
                              - GET <id:number>: sets a value on a key.
                              - DEL <id:number>: deletes a value on a key.
                              - help: how this message.
                              - clear: clear prompt.
                              - exit: exit repl.
                        "},
                        "clear" => print!("\x1B[2J\x1B[1;1H"),
                        "exit" => process::exit(0),
                        _ => {
                            match stream.write_all(format!("{}\n", line).as_bytes()) {
                                Ok(_) => handle_reply!(stream),
                                Err(err) => write_error!(format!("Failed: {err}"))
                            }
                        }
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
