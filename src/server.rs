use std::{collections::HashMap, io, process, sync::Arc};
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
    sync::Mutex,
};
use indoc::indoc;

macro_rules! write_error {
    ($stream:expr, $message:expr) => {
        {
            $stream.write_all(format!("err {}\n", $message).as_bytes()).await.unwrap();
            continue;
        }
    };
}

macro_rules! write_ok {
    ($stream:expr, $message:expr) => {
        {
            $stream.write_all(format!("ok {}\n", $message).as_bytes()).await.unwrap();
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

pub async fn start(addr: &str, debug: bool) {
    let listener = match TcpListener::bind(addr).await {
        Ok(listener) => listener,
        Err(err) => {
            println!("Failed to start the server: {err}");
            process::exit(1);
        }
    };

    let local_addr = &listener.local_addr().unwrap();

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

    loop {
        let mut stream = match listener.accept().await {
            Ok((stream, _)) => stream,
            Err(err) => {
                println!("Failed to get the client: {err}");
                return;
            }
        };

        let db_clone = Arc::clone(&db);

        tokio::spawn(async move {
            handle_connection(&mut stream, db_clone, debug).await;
        });
    }
}

async fn handle_connection(stream: &mut TcpStream, db_clone: Arc<Mutex<HashMap<u32, String>>>, debug: bool) {
    loop {
        let mut buffer = [0; 4096];
        let line = match stream.try_read(&mut buffer) {
            Ok(0) => {
                debug!("Connection closed.", debug);
                break;
            },
            Ok(bytes) => std::str::from_utf8(&buffer[..bytes]).unwrap(),
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue,
            Err(err) => {
                println!("Failed to read: {err}.");
                continue;
            }
        };

        if line.trim().is_empty() {
            continue;
        }

        let mut parts = line.splitn(3, ' ').map(|part| part.trim());

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
                let db = db_clone.lock().await;
                let result = match db.get(&id) {
                    Some(item) => item,
                    None => write_error!(stream, format!("Cannot find item with an id of \"{id}\""))
                };

                write_ok!(stream, format!("{:?}\n", result));
            },
            "SET" => {
                let mut db = db_clone.lock().await;
                db.insert(id, data);

                write_ok!(stream, "");
            },
            "DEL" => {
                let mut db = db_clone.lock().await;
                db.remove(&id);

                write_ok!(stream, "");
            }
            _ => write_error!(stream, "Invalid command.")
        }
    }
}
