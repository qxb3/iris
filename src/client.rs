use std::{io::Write, net::TcpStream};

pub fn start(addr: &str) {
    match TcpStream::connect(addr) {
        Ok(mut stream) => {
            stream.write_all("test\nfoo".as_bytes()).ok();
        }
        Err(err) => panic!("failed to connect: {err}"),
    }
}
