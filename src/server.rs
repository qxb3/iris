use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    process,
    sync::{Arc, Mutex},
    thread
};

use indoc::indoc;

macro_rules! write_error {
    ($stream:expr, $message:expr) => {
        {
            $stream.write_all(format!("err {}\n", $message).as_bytes()).unwrap();
            continue;
        }
    };
}

macro_rules! write_ok {
    ($stream:expr, $message:expr) => {
        {
            $stream.write_all(format!("ok {}\n", $message).as_bytes()).unwrap();
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

                thread::spawn(move || handle_connection(incoming.unwrap(), &db_clone, debug));
            }

            drop(listener);
        }
        Err(err) => {
            println!("Failed to start server at {addr}: {err}");
            process::exit(1);
        }
    }
}

fn handle_connection(mut stream: TcpStream, db_clone: &Arc<Mutex<HashMap<u32, String>>>, debug: bool) {
    let mut db = db_clone.lock().unwrap();

    loop {
        let mut buf_reader = BufReader::new(&mut stream);

        let mut buffer = String::new();
        match buf_reader.read_line(&mut buffer) {
            Ok(byte) => {
                if byte == 0 {
                    debug!("Connection closed.", debug);
                    break;
                }
            },
            Err(err) => {
                write_error!(stream, format!("Failed to read stream: {err}"));
            }
        }

        if buffer.is_empty() {
            continue;
        }

        let mut parts = buffer.splitn(3, ' ').map(|part| part.trim());

        let command = match parts.next() {
            Some(command) => command.to_uppercase(),
            None => write_error!(stream, "No command specified.")
        };

        let id = match parts.next() {
            Some(id) => match id.parse::<u32>() {
                Ok(id) => id,
                Err(_) => write_error!(stream, "ID needs to be a number.")
            },
            None => write_error!(stream, "No ID specified.")
        };

        let data = parts.collect::<Vec<&str>>().join(" ");
        if data.len() <= 0 && command == "SET" || command == "UPD" {
            write_error!(stream, format!("<data> is required for \"{command}\""));
        }

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
                    None => write_error!(stream, format!("Cannot find item with an id of \"{id}\""))
                };

                write_ok!(stream, result);
            },
            "SET" => {
                db.insert(id, data);
                write_ok!(stream, "");
            }
            _ => write_error!(stream, "Invalid command.")
        }
    }
}
