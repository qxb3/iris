use std::{
    io::{BufRead, BufReader, Write}, net::TcpListener, process, thread
};

use indoc::indoc;

pub fn start(addr: &str) {
    match TcpListener::bind(addr) {
        Ok(listener) => {
            let local_addr = listener.local_addr().unwrap();

            println!(
                indoc! {"
                      ▀             ▀
                    ▄▄▄     ▄ ▄▄  ▄▄▄     ▄▄▄
                      █     █▀  ▀   █    █   ▀
                      █     █       █     ▀▀▀▄
                    ▄▄█▄▄   █     ▄▄█▄▄  ▀▄▄▄▀

                    Server has started.
                    • version:  {},
                    • host:     http://{}
                    • port:     {}
                "},
                env!("CARGO_PKG_VERSION"),
                local_addr,
                local_addr.port()
            );

            for incoming in listener.incoming() {
                thread::spawn(|| {
                    let mut stream = incoming.unwrap();
                    let buf_reader = BufReader::new(&mut stream);
                    let req: Vec<String> = buf_reader
                        .lines()
                        .map(|result| result.unwrap())
                        .take_while(|line| !line.is_empty())
                        .collect();

                    println!("Req: {:#?}", req);

                    stream
                        .write_all("HTTP/1.1 200 OK\r\n\r\n".as_bytes())
                        .unwrap();
                });
            }

            drop(listener);
        },
        Err(err) => {
            println!("Failed to start server at {addr}: {err}");
            process::exit(1);
        }
    }
}
