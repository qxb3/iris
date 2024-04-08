use std::{io::{BufRead, BufReader, Write}, net::TcpStream, process};

pub fn start(addr: &str) {
    match TcpStream::connect(addr) {
        Ok(mut stream) => {
            loop {
                if let Ok(line) = readline() {
                    if line.is_empty() {
                        continue;
                    }

                    let parts: Vec<&str> = line.split(' ')
                        .filter(|part| !part.is_empty())
                        .map(|part| part.trim())
                        .collect();

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
                            let command = parts[0].to_uppercase();
                            let id = match parts[1].parse::<u32>() {
                                Ok(part) => part,
                                Err(err) => {
                                    println!("ID needs to be a number: {err}.");
                                    continue;
                                }
                            };
                            let data = parts[2];

                            match stream.write_all(format!("Command: {}\nID: {}\nData: {}\r\n\r\n", command, id, data).as_bytes()) {
                                Ok(_) => {
                                    let buf_reader = BufReader::new(&mut stream);
                                    let reply: Vec<String> = buf_reader
                                        .lines()
                                        .map(|result| result.unwrap())
                                        .take_while(|line| !line.is_empty())
                                        .collect();

                                    println!("Reply: {:#?}", reply);
                                },
                                Err(err) => println!("Failed: {err}")
                            }
                        },
                        _ => println!("Too many arguments.")
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
