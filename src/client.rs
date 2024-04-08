use std::{io::Write, net::TcpStream, process};

pub fn start(addr: &str) {
    match TcpStream::connect(addr) {
        Ok(mut _stream) => {
            loop {
                if let Ok(line) = readline() {
                    if line.is_empty() {
                        continue;
                    }

                    let parts: Vec<&str> = line.split(' ').collect();

                    match parts.len() {
                        0 => println!("No command given."),
                        1 => {
                            let part = parts[0];

                            match part {
                                "help" => println!("help menu"),
                                "exit" => process::exit(0),
                                _ => println!("No command given.")
                            }
                        },
                        2 => println!("No data provided."),
                        3 => {
                            let command = parts[0];
                            let id = match parts[1].parse::<u32>() {
                                Ok(part) => part,
                                Err(err) => {
                                    println!("ID needs to be a number: {err}.");
                                    process::exit(1);
                                }
                            };
                            let data = parts[2];

                            match command {
                                "get" => println!("get"),
                                "set" => println!("set"),
                                "upd" => println!("update"),
                                "del" => println!("delete"),
                                _ => println!("Invalid command.")
                            }
                        },
                        _ => println!("Too many arguments.")
                    }
                }
            }

            // stream.write_all("Command: SET\nID: 1\nData: {\"foo\": true}".as_bytes()).ok();
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
