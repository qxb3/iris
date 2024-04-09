use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    net::TcpListener,
    process,
    sync::{Arc, Mutex},
    thread,
};

use indoc::indoc;

macro_rules! write_error {
    ($stream:expr, $message:expr) => {
        {
            $stream.write_all(format!("{}\n", $message).as_bytes()).unwrap();
            continue;
        }
    };
}

macro_rules! debug {
    ($message:expr, $condition:expr) => {
        if $condition {
            println!("{}", $message);
        }
    };
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
                    • version:  {}
                    • host:     http://{}
                    • port:     {}
                "},
                env!("CARGO_PKG_VERSION"),
                local_addr,
                local_addr.port()
            );

            let db: Arc<Mutex<HashMap<u32, String>>> = Arc::new(Mutex::new(HashMap::new()));

            for incoming in listener.incoming() {
                let db_clone = Arc::clone(&db);

                thread::spawn(move || {
                    let mut db = db_clone.lock().unwrap();

                    let mut stream = incoming.unwrap();

                    loop {
                        let mut buf_reader = BufReader::new(&mut stream);

                        let mut buffer = String::new();
                        match buf_reader.read_line(&mut buffer) {
                            Ok(byte) => {
                                if byte == 0 {
                                    debug!("Connection closed", debug);
                                    break;
                                }

                                let mut parts = buffer
                                    .splitn(3, ' ')
                                    .map(|part| part.trim());

                                let command = match parts.next() {
                                    Some(command) => command.to_uppercase(),
                                    None => write_error!(stream, "err No command specified")
                                };

                                let id = match parts.next() {
                                    Some(id) => match id.parse::<u32>() {
                                        Ok(id) => id,
                                        Err(_) => write_error!(stream, "err ID needs to be a number")
                                    },
                                    None => write_error!(stream, "err No ID specified. \"<command> <id> [data]\"")
                                };

                                let data: Option<String> = match parts.next() {
                                    Some(data) => Some(data.to_string()),
                                    None => {
                                        if command == "SET" || command == "UPD" {
                                            write_error!(stream, format!("err <data> is required for \"{command}\". <command> <id> <data>"))
                                        } else {
                                            None
                                        }
                                    }
                                };

                                debug!(format!(
                                    indoc! {"
                                        Request:
                                            - Command: {}
                                            - ID:      {}
                                            - Data:    {:?}
                                    "},
                                    command,
                                    id,
                                    data
                                ), debug);

                                match command.as_str() {
                                    "GET" => {
                                        let result = match db.get(&id) {
                                            Some(item) => item,
                                            None => write_error!(stream, format!("err Cannot find item with an id of \"{id}\""))
                                        };

                                        stream.write_all(format!("ok {result}\n").as_bytes()).unwrap();
                                    },
                                    "SET" => {
                                        db.insert(id, data.unwrap());
                                        stream.write_all("ok\n".as_bytes()).unwrap();
                                    }
                                    _ => {}
                                }
                            },
                            Err(err) => println!("Failed: {err}")
                        }
                    }
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
