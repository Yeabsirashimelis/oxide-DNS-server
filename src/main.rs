mod server;
mod dns;
// mod utils;

fn main() {
    server::udp::start("127.0.0.1:2053");
}
