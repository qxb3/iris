mod server;
mod client;

fn main() {
    let addr = "127.0.0.1:3000";

    server::start(addr);
    client::start(addr);
}
