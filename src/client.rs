use std::{io::Write, net::TcpStream, process};

pub fn start(addr: &str) {
    match TcpStream::connect(addr) {
        Ok(mut stream) => {
            stream.write_all("test\nfoo".as_bytes()).ok();
        }
        Err(err) => {
            println!("Failed to connect: {err}");
            process::exit(1);
        },
    }
}
