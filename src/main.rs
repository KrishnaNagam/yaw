use std::{thread, time::Duration};

use tokio;
use yaw::{
    config::Config,
    http::{request::Method, response::Status},
    server::Server,
};

use hyper::Client;
use url_builder::URLBuilder;

#[tokio::main]
async fn main() {
    let mut server = match Server::new() {
        Ok(server) => server,
        Err(e) => panic!("error while creating the server"),
    };
    let mut config = Config::default();
    config.set_index("hello.html");
    config.add_request_handler("/sleep", |request, response| {
        thread::sleep(Duration::from_secs(
            request
                .get_param("time")
                .unwrap_or(&"5".to_string())
                .parse()
                .unwrap_or(0), //5
        ));
        response.set_status_code(Status::Ok);
        response.set_body("slept\r\n".into());
    });

    config.add_request_handler("/hello", |request, response| {
        response.set_status_code(Status::Ok);
        response.set_body(format!("Hello {} \r\n",request.get_param("name").unwrap_or(&"None".to_string())));
    });

    config.add_request_handler("/login", |request, response| {
        let mut url = URLBuilder::new();
        url.set_protocol("https")
        .set_host("accounts.google.com")
        .add_route("/o/oauth2/v2/auth")
        .add_param("response_type", "code")
        .add_param("client_id", "1036455972843-v01j1391142k91k230hm49f4ppbgq183.apps.googleusercontent.com")
        .add_param("scope", "openid email")
        .add_param("redirect_uri", "http://lvh.me:8080/callback")
        .add_param("state", "some token")
        .add_param("nonce", "0394852-3190485-2490358");
        response.set_status_code(Status::Found);
        response.add_header("Location",&url.build())
    });

    server.set_config(config);
    server.run().await;
    print!("server is running")
}
