use indoc::indoc;
use std::{collections::HashMap, io, process, sync::Arc};
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
    sync::Mutex,
};

use crate::core::iris_command::{parse_command, Command, Expr};

macro_rules! write_error {
    ($stream:expr, $message:expr) => {{
        $stream
            .write_all(format!("err {}\n", $message).as_bytes())
            .await
            .unwrap();
        continue;
    }};
}

macro_rules! write_ok {
    ($stream:expr, $message:expr) => {{
        $stream
            .write_all(format!("{}\n", $message).as_bytes())
            .await
            .unwrap();
    }};
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

async fn handle_connection(
    stream: &mut TcpStream,
    db_clone: Arc<Mutex<HashMap<u32, String>>>,
    debug: bool,
) {
    loop {
        let mut buffer = [0; 4096];
        let line = match stream.try_read(&mut buffer) {
            Ok(0) => {
                debug!("Connection closed.", debug);
                break;
            }
            Ok(bytes) => std::str::from_utf8(&buffer[..bytes]).unwrap().trim(),
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue,
            Err(err) => {
                println!("Failed to read: {err}.");
                continue;
            }
        };

        if line.is_empty() {
            continue;
        }

        let command = parse_command(line);

        debug!(
            format!(
                indoc! {"
                Request:
                - Command: {:?}
            "},
                &command,
            ),
            debug
        );

        match command {
            Command::Get { id } => {
                let db = db_clone.lock().await;
                let result = match db.get(&id) {
                    Some(item) => item,
                    None => {
                        write_error!(stream, format!("Cannot find item with an id of \"{id}\""))
                    }
                };

                write_ok!(stream, format!("{:?}\n", result));
            }
            Command::List { expr } => {
                let db = db_clone.lock().await;

                match expr {
                    Expr::Number(mut count) => {
                        if count == -1 {
                            count = db.len() as i32;
                        }

                        let result: Vec<(&u32, &String)> = db.iter().take(count as usize).collect();

                        write_ok!(stream, format!("{:?}", result));
                    }
                    Expr::Range(start, mut end) => {
                        if end < 0 {
                            end = db.len() as i32;
                        }

                        let result: Vec<(&u32, &String)> = db
                            .iter()
                            .skip(start as usize)
                            .take((end + 1) as usize)
                            .collect();

                        write_ok!(stream, format!("{:?}", result));
                    }
                }
            }
            Command::Count { expr } => {
                let db = db_clone.lock().await;

                match expr {
                    Expr::Number(mut count) => {
                        if count == -1 {
                            count = db.len() as i32;
                        }

                        let result: Vec<(&u32, &String)> = db.iter().take(count as usize).collect();

                        write_ok!(stream, format!("{}", result.len()));
                    }
                    Expr::Range(start, mut end) => {
                        if end < 0 {
                            end = db.len() as i32;
                        }

                        let result: Vec<(&u32, &String)> = db
                            .iter()
                            .skip(start as usize)
                            .take((end + 1) as usize)
                            .collect();

                        write_ok!(stream, format!("{}", result.len()));
                    }
                }
            }
            Command::Set { id, data } => {
                let mut db = db_clone.lock().await;
                db.insert(id, data);

                write_ok!(stream, format!("{}", id));
            }
            Command::Delete { expr } => {
                let mut db = db_clone.lock().await;

                match expr {
                    Expr::Number(id) => {
                        match db.remove(&(id as u32)) {
                            Some(data) => write_ok!(stream, data),
                            None => write_error!(stream, format!("Cannot delete item with an id of {id}"))
                        }
                    }
                    Expr::Range(start, mut end) => {
                        if end < 0 {
                            end = (db.len() - 1) as i32;
                        }

                        let mut result = String::new();
                        for id in start..end + 1 {
                            if let Some(data) = db.remove(&(id as u32)) {
                                result.push_str(format!("{data} ").as_str());
                            }
                        }

                        write_ok!(
                            stream,
                            format!(
                                "[{}]",
                                result
                                    .trim()
                                    .split_whitespace()
                                    .collect::<Vec<&str>>()
                                    .join(", ")
                                    .trim()
                                    .to_string()
                            )
                        );
                    }
                }
            }
            Command::Invalid { reason } => write_error!(stream, reason),
        }
    }
}
