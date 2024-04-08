mod client;
mod server;

fn main() {
    server::start("127.0.0.1:3000");
}
