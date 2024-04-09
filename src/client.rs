use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
    process,
};

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

                    match stream.write_all(format!("{}\n", line).as_bytes()) {
                        Ok(_) => handle_reply!(stream),
                        Err(err) => write_error!(format!("Failed: {err}"))
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
