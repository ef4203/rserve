

mod http;
mod server;

fn main() {
    server::Server::run(8080);
}
