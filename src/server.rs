use indoc::indoc;
use std::{collections::HashMap, io, process, sync::Arc};
use serde_json::json;
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
    sync::Mutex,
};

use crate::command::{parse_command, Command, Expr};

macro_rules! respond_command_ok {
    ($format:expr, $message:expr) => {
        match $format.as_str() {
            "default" => Ok($message),
            "json" => Ok(json!({ "status": "ok", "response": $message }).to_string()),
            _ => unreachable!()
        }
    };
}

macro_rules! respond_command_err {
    ($format:expr, $message:expr) => {
        match $format.as_str() {
            "default" => Ok($message),
            "json" => Ok(json!({ "status": "err", "response": $message }).to_string()),
            _ => unreachable!()
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

pub async fn start(addr: &str, format: String, debug: bool) {
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

    let db: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let mut stream = match listener.accept().await {
            Ok((stream, _)) => stream,
            Err(err) => {
                println!("Failed to get the client: {err}");
                return;
            }
        };

        let format_clone = format.clone();
        let db_clone = Arc::clone(&db);

        tokio::spawn(async move {
            handle_connection(&mut stream, db_clone, format_clone, debug).await;
        });
    }
}

async fn handle_connection(
    stream: &mut TcpStream,
    db_clone: Arc<Mutex<HashMap<String, String>>>,
    format: String,
    debug: bool
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
        match handle_command(&command, &format, &db_clone).await {
            Ok(resp) => match format.as_str() {
                "default" => { stream.write(format!("ok {resp}\n").as_bytes()).await.unwrap(); },
                "json" => { stream.write(format!("{resp}\n").as_bytes()).await.unwrap(); },
                _ => unreachable!()
            },
            Err(err) => match format.as_str() {
                "default" => { stream.write(format!("err {err}\n").as_bytes()).await.unwrap(); },
                "json" => { stream.write(format!("{err}\n").as_bytes()).await.unwrap(); },
                _ => unreachable!()
            },
        }

        debug!(
            format!(
                indoc! {"
                Request:
                - Command: {:?} => {:?}
            "},
                &line,
                &command
            ),
            debug
        );
    }
}

async fn handle_command<'a>(
    command: &Command<'a>,
    format: &String,
    db_clone: &Arc<Mutex<HashMap<String, String>>>
) -> Result<String, String> {
    match command {
        Command::Get { id } => {
            let db = db_clone.lock().await;
            let result = match db.get(id) {
                Some(item) => item,
                None => return respond_command_err!(format, format!("Cannot find item with an id of {id}"))
            };

            respond_command_ok!(format, result.to_owned())
        }
        Command::List { expr } => {
            let db = db_clone.lock().await;

            match expr {
                Expr::Number(mut count) => {
                    if count == -1 {
                        count = db.len() as i32;
                    }

                    let result: Vec<(String, String)> = db
                        .iter()
                        .take(count as usize)
                        .map(|(id, data)| (id.to_owned(), data.to_owned()))
                        .collect();

                    respond_command_ok!(format, format!("{:?}", result))
                }
                Expr::Range(start, mut end) => {
                    if end < 0 {
                        end = db.len() as i32;
                    }

                    let result: Vec<(String, String)> = db
                        .iter()
                        .skip(*start as usize)
                        .take((end + 1) as usize)
                        .map(|(id, data)| (id.to_owned(), data.to_owned()))
                        .collect();

                    respond_command_ok!(format, format!("{:?}", result))
                },
                _ => respond_command_err!(format, "This is expression is not allowed".to_string())
            }
        }
        Command::Count { expr } => {
            let db = db_clone.lock().await;

            match expr {
                Expr::Number(mut count) => {
                    if count == -1 {
                        count = db.len() as i32;
                    }

                    let result: Vec<(String, String)> = db
                        .iter()
                        .take(count as usize)
                        .map(|(id, data)| (id.to_owned(), data.to_owned()))
                        .collect();

                    respond_command_ok!(format, format!("{}", result.len()))
                }
                Expr::Range(start, mut end) => {
                    if end < 0 {
                        end = db.len() as i32;
                    }

                    let result: Vec<(String, String)> = db
                        .iter()
                        .skip(*start as usize)
                        .take((end + 1) as usize)
                        .map(|(id, data)| (id.clone(), data.clone()))
                        .collect();

                    respond_command_ok!(format, format!("{}", result.len()))
                },
                _ => respond_command_err!(format, "This is expression is not allowed".to_string())
            }
        }
        Command::Set { id, data } => {
            let mut db = db_clone.lock().await;

            db.insert(id.to_owned(), data.to_owned());

            respond_command_ok!(format, id.to_owned())
        }
        Command::Delete { expr } => {
            let mut db = db_clone.lock().await;

            match expr {
                Expr::ID(id) => {
                    match db.remove(id) {
                        Some(data) => respond_command_ok!(format, data),
                        None => respond_command_err!(format, format!("Cannot delete item with an id of {:?}", id))
                    }
                },
                Expr::Number(mut count) => {
                    if count == -1 {
                        count = db.len() as i32;
                    }

                    let mut result = vec![];
                    let items: Vec<(String, String)> = db
                        .iter()
                        .take(count as usize)
                        .map(|(id, data)| (id.clone(), data.clone()))
                        .collect();

                    for (id, data) in items {
                        db.remove(&id);
                        result.push((id, data));
                    }

                    respond_command_ok!(format, format!("{:?}", result))
                },
                Expr::Range(start, mut end) => {
                    if end < 0 {
                        end = (db.len() - 1) as i32;
                    }

                    let mut result = vec![];
                    let items: Vec<(String, String)> = db
                        .iter()
                        .skip(*start as usize)
                        .take((end + 1) as usize)
                        .map(|(id, data)| (id.clone(), data.clone()))
                        .collect();

                    for (id, data) in items {
                        db.remove(&id);
                        result.push((id, data));
                    }

                    respond_command_ok!(format, format!("{:?}", result))
                },
            }
        }
        Command::Invalid { reason } => respond_command_err!(format, reason.to_string()),
    }
}
