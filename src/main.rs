use http::server::Server;

mod http;
mod config;

fn main() {
    let mut server = Server::new();
    let mut config = http::server::Config::new();
    config.set_index("hello.html");

    server.set_config(config);
    server.run();
}