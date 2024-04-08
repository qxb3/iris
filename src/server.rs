use std::{
    io::{BufRead, BufReader, Write},
    net::TcpListener,
    thread,
};

pub fn start(addr: &str) {
    let listener = TcpListener::bind(addr).unwrap();

    for incoming in listener.incoming() {
        thread::spawn(|| {
            let mut stream = incoming.unwrap();
            let buf_reader = BufReader::new(&mut stream);
            let req: Vec<String> = buf_reader
                .lines()
                .map(|result| result.unwrap())
                .take_while(|line| !line.is_empty())
                .collect();

            println!("Req: {:#?}", req);

            stream
                .write_all("HTTP/1.1 200 OK\r\n\r\n".as_bytes())
                .unwrap();
        });
    }
}
