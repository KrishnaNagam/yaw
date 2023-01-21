use yaw::{server::Server, config::Config};


fn main() {
    let mut server = Server::new();
    let mut config = Config::new();
    config.set_index("hello.html");

    server.set_config(config);
    server.run();
}