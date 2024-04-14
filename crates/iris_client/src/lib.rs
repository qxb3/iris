mod pipe_builder;

use std::ops::Range;
use pipe_builder::PipeBuilder;
use regex::Regex;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

#[derive(Debug)]
pub enum Expression {
    Number(i32),
    Range(Range<i32>)
}

#[derive(Debug)]
pub enum DeleteExpression<'a> {
    Number(i32),
    ID(&'a str),
    Range(Range<i32>)
}

#[derive(Debug)]
pub struct ServerResponse {
    pub status: String,
    pub data: String,
}

#[derive(Debug)]
pub struct Item {
    pub id: String,
    pub data: String
}

#[derive(Debug)]
pub struct IrisClient {
    socket: TcpStream,
}

impl IrisClient {
    pub async fn set(self: &mut Self, id: &str, data: &str) -> Result<ServerResponse, String> {
        self.socket
            .write_all(format!("SET {id} {data}\n").as_bytes())
            .await
            .map_err(|err| format!("Failed to send the command: {err}"))?;

        let server_resp = self.server_response().await?;
        Ok(server_resp)
    }

    pub async fn delete<'a>(self: &mut Self, expr: DeleteExpression<'a>) -> Result<Vec<Item>, String> {
        match expr {
            DeleteExpression::Number(count) => {
                self.socket
                    .write_all(format!("DEL {count}\n").as_bytes())
                    .await
                    .map_err(|err| format!("Failed to send the command: {err}"))?;
            }
            DeleteExpression::ID(id) => {
                self.socket
                    .write_all(format!("DEL {id}\n").as_bytes())
                    .await
                    .map_err(|err| format!("Failed to send the command: {err}"))?;
            }
            DeleteExpression::Range(range) => {
                self.socket
                    .write_all(format!("DEL {:?}\n", range).as_bytes())
                    .await
                    .map_err(|err| format!("Failed to send the command: {err}"))?;
            }
        }

        let server_resp = self.server_response().await?;
        let deleted = self.parse_tuple(server_resp.data.as_str())?;

        Ok(deleted)
    }

    pub async fn get(self: &mut Self, id: &str) -> Result<ServerResponse, String> {
        self.socket
            .write_all(format!("GET {id}\n").as_bytes())
            .await
            .map_err(|err| format!("Failed to send the command: {err}"))?;

        let server_resp = self.server_response().await?;
        Ok(server_resp)
    }

    pub async fn list(self: &mut Self, expr: Expression) -> Result<Vec<Item>, String> {
        match expr {
            Expression::Number(count) => {
                self.socket
                    .write_all(format!("LST {count}\n").as_bytes())
                    .await
                    .map_err(|err| format!("Failed to send the command: {err}"))?;
            }
            Expression::Range(range) => {
                self.socket
                    .write_all(format!("LST {:?}\n", range).as_bytes())
                    .await
                    .map_err(|err| format!("Failed to send the command: {err}"))?;
            }
        }

        let server_resp = self.server_response().await?;
        let list = self.parse_tuple(server_resp.data.as_str()).unwrap();

        Ok(list)
    }

    pub async fn count(self: &mut Self, expr: Expression) -> Result<u32, String> {
        match expr {
            Expression::Number(count) => {
                self.socket
                    .write_all(format!("CNT {count}\n").as_bytes())
                    .await
                    .map_err(|err| format!("Failed to send the command: {err}"))?;
            }
            Expression::Range(range) => {
                self.socket
                    .write_all(format!("CNT {:?}\n", range).as_bytes())
                    .await
                    .map_err(|err| format!("Failed to send the command: {err}"))?;
            }
        }

        let server_resp = self.server_response().await?;
        let count = str::parse::<u32>(server_resp.data.as_str()).unwrap();

        Ok(count)
    }

    pub async fn raw(self: &mut Self, command: &str) -> Result<ServerResponse, String> {
        self.socket
            .write_all(format!("{command}\n").as_bytes())
            .await
            .map_err(|err| format!("Failed to send the command: {err}"))?;

        let server_resp = self.server_response().await?;
        Ok(server_resp)
    }

    pub fn pipe(self: &mut Self) -> PipeBuilder {
        PipeBuilder {
            command: String::new(),
            client: self
        }
    }

    async fn server_response(self: &mut Self) -> Result<ServerResponse, String> {
        let mut buf_reader = BufReader::new(&mut self.socket);
        let mut buffer = String::new();
        let server_resp = match buf_reader.read_line(&mut buffer).await {
            Ok(0) => return Err("Connection closed".to_string()),
            Ok(_) => {
                let response = self.parse_response(buffer.trim().to_string());

                if response.status == "err" {
                    return Err(response.data);
                }

                response
            }
            Err(err) => return Err(format!("Failed to read server response: {err}")),
        };

        Ok(server_resp)
    }

    fn parse_response(&self, response: String) -> ServerResponse {
        let parts: Vec<&str> = response.splitn(2, ' ').collect();

        ServerResponse {
            status: parts.get(0).unwrap().to_string(),
            data: parts.get(1).unwrap().to_string(),
        }
    }

    fn parse_tuple(&self, response: &str) -> Result<Vec<Item>, String> {
        let regex = Regex::new(r#"\s*\[\s*(\(".*?",\s*".*?"\)\s*,?\s*)*\]\s*"#).unwrap();

        if !regex.is_match(response) {
            return Err("Invalid tuple response".to_string());
        }

        let mut result = Vec::new();

        let pairs = Regex::new(r#"\("(.*?)",\s*"(.*?)"\)"#).unwrap();
        for cap in pairs.captures_iter(response) {
            let id = cap.get(1).unwrap().as_str().to_string();
            let data = cap.get(2).unwrap().as_str().to_string();

            result.push(Item {
                id,
                data
            });
        }

        Ok(result)
    }
}

pub async fn connect(addr: &str) -> Result<IrisClient, String> {
    let socket = TcpStream::connect(addr)
        .await
        .map_err(|err| format!("Failed to connect: {err}"))?;

    Ok(IrisClient { socket })
}
