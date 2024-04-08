use std::{io::Write, net::TcpStream};

pub fn start(addr: &str) {
    match TcpStream::connect(addr) {
        Ok(mut stream) => {
            for i in 0..100 {
                stream.write_all("fuck you\nHAHAHAHAQQ".as_bytes()).ok();
            }
        }
        Err(err) => panic!("failed to connect: {err}"),
    }
}
