use indoc::{indoc, printdoc};
use std::process;
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

pub async fn start(addr: &str) {
    let mut stream = match TcpStream::connect(addr).await {
        Ok(stream) => stream,
        Err(err) => {
            println!("Failed to connect: {err}");
            process::exit(1);
        }
    };

    let local_addr = stream.peer_addr().unwrap();

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
        let line = match prompt().await {
            Ok(line) => line,
            Err(err) => {
                println!("{err}");
                process::exit(1);
            }
        };

        if line.is_empty() {
            continue;
        }

        let command = line
            .clone()
            .splitn(3, ' ')
            .collect::<Vec<&str>>()
            .first()
            .unwrap()
            .to_lowercase();

        match command.as_str() {
            "help" => printdoc! {"
                • What is iris?
                  iris is a simple key value database,
                  every value in iris is considered to be a string (for now)
                  and you, yourself will be the one to parse the types.

                • Commands
                  <id>   = string.
                  <expr> = number | <id> | range (0..5).
                  <data> = string.

                 - GET <id>           : gets a value on a key.
                 - LST <expr>         : list keys and its value based on expression.
                 - CNT <expr>         : count all values.
                 - SET <expr> <data>  : sets a value on a key.
                 - APP <id> <data>    : appends a data on id.
                 - DEL <expr>         : deletes a value on a key.
                 - help               : show this message.
                 - clear              : clear prompt.
                 - exit               : exit repl.
            "},
            "clear" => print!("\x1B[2J\x1B[1;1H"),
            "exit" => process::exit(0),
            _ => match stream.write_all(format!("{line}\n").as_bytes()).await {
                Ok(_) => {
                    let mut buf_reader = BufReader::new(&mut stream);
                    let mut buffer = [0; 4096];

                    let server_resp = match buf_reader.read(&mut buffer).await {
                        Ok(0) => {
                            println!("Connection closed.");
                            process::exit(1);
                        }
                        Ok(byte) => String::from_utf8_lossy(&buffer[..byte]),
                        Err(err) => {
                            println!("Failed to read: {err}.");
                            process::exit(1);
                        }
                    };

                    println!("> {server_resp}");
                }
                Err(err) => {
                    println!("Failed to send: {err}");
                    process::exit(1);
                }
            },
        }
    }
}

async fn prompt() -> Result<String, String> {
    let mut stdin = BufReader::new(tokio::io::stdin());
    let mut stdout = tokio::io::stdout();

    stdout
        .write(format!("iris@{} $ ", env!("CARGO_PKG_VERSION")).as_bytes())
        .await
        .map_err(|e| format!("Failed to write: {e}"))?;

    stdout
        .flush()
        .await
        .map_err(|e| format!("Failed to flush: {e}"))?;

    let mut buffer = String::new();
    let line = match stdin.read_line(&mut buffer).await {
        Ok(0) => return Err("Connection to stdout closed".to_string()),
        Ok(_) => buffer.trim().to_string(),
        Err(err) => return Err(format!("Failed to read input: {err}")),
    };

    Ok(line)
}
