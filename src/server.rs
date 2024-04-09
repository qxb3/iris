use std::{
    collections::HashMap, hash::Hash, io::{BufRead, BufReader, Write}, net::TcpListener, process, sync::{Arc, Mutex}, thread
};

use indoc::indoc;

macro_rules! write_error {
    ($stream:expr, $data:expr) => {
        {
            let error_message = format!("Status: Err\nData: {:?}\r\n\r\n", $data);
            $stream.write_all(error_message.as_bytes()).unwrap();
            return;
        }
    };
}

trait ParseLine {
    fn parse_line(&self) -> Option<(String, String)>;
}

impl ParseLine for String {
    fn parse_line(&self) -> Option<(String, String)> {
        self
            .split_once(':')
            .map(|(key, value)| (key.trim().to_string(), value.trim().to_string()))
    }
}

pub fn start(addr: &str, debug: bool) {
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

            let iris: Arc<Mutex<HashMap<u32, String>>> = Arc::new(Mutex::new(HashMap::new()));

            for incoming in listener.incoming() {
                let iris_clone = Arc::clone(&iris);

                thread::spawn(move || {
                    let mut iris = iris_clone.lock().unwrap();

                    let mut stream = incoming.unwrap();
                    let buf_reader = BufReader::new(&mut stream);
                    let request: HashMap<String, String> = buf_reader
                        .lines()
                        .map(|result| result.unwrap())
                        .take_while(|line| line != "End")
                        .filter(|line| line.parse_line().is_some())
                        .map(|line| {
                            let (key, value) = line.parse_line().unwrap();
                            (key.to_lowercase(), value)
                        })
                        .collect();

                    println!("{:?}", request);

                    if debug {
                        println!("Request: {:#?}", request);
                    }

                    let command = match request.get("command") {
                        Some(command) => command.to_uppercase(),
                        None => write_error!(stream, "Cannot find \"Command\"")
                    };

                    let id = match request.get("id") {
                        Some(id) => match id.parse::<u32>() {
                            Ok(id) => id,
                            Err(_) => write_error!(stream, "ID needs to be a number")
                        },
                        None => write_error!(stream, "Cannot find \"ID\"")
                    };

                    let data: Option<String> = match request.get("data") {
                        Some(data) => Some(data.to_owned()),
                        None => {
                            if command == "SET" || command == "UPD" {
                                write_error!(stream, format!("\"Data\" is required for \"{command}\""))
                            } else {
                                None
                            }
                        }
                    };

                    println!("{command} {id} {:?}", data);

                    stream
                        .write_all("Reply Babyyyyyyy\nEnd\n".as_bytes())
                        .unwrap();
                });
            }

            drop(listener);
        }
        Err(err) => {
            println!("Failed to start server at {addr}: {err}");
            process::exit(1);
        }
    }
}
