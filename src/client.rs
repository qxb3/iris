use std::{io::{self, Write}, process};
use tokio::{io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, net::TcpStream};
use indoc::{indoc, printdoc};

pub async fn start(addr: &str) {
    let mut stream = match TcpStream::connect(addr).await {
        Ok(stream) => stream,
        Err(err) => {
            println!("Failed to connect: {err}");
            process::exit(1);
        }
    };

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
        let line = match readline() {
            Ok(line) => line,
            Err(err) => {
                println!("Failed prompt: {err}");
                process::exit(1);
            }
        };

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
            _ => {}
        }

        if let Err(err) = stream.write_all(format!("{line}\n").as_bytes()).await {
            println!("Failed to send: {err}");
            continue;
        }

        let mut buffer = String::new();
        let server_resp = match BufReader::new(&mut stream).read_line(&mut buffer).await {
            Ok(0) => {
                println!("Connection closed.");
                process::exit(1);
            },
            Ok(_) => buffer,
            Err(err) => {
                println!("> Failed to read stream: {err}");
                continue;
            }
        };

        println!("> {server_resp}");
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
