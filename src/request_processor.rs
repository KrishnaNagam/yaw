use std::{fs, sync::Arc, thread, time::Duration};

use crate::{
    http::{
        auth::basic_auth_validate,
        request::{Method, Request},
        response,
    }, config::Config,
};

pub struct RequestProcessor {
    config: Arc<Config>,
}

impl RequestProcessor {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config: config }
    }

    pub fn process(self, request: Request) -> response::Response {
        let mut response = response::Response::new();

        self.handle_authentication(&request, &mut response);
        self.handle_custom_routes(&request, &mut response);
        self.handle_everything_else(&request, &mut response);

        response
    }

    fn handle_authentication(&self, request: &Request, response: &mut response::Response) {
        let root_path = &self.config.root_path;

        if response.get_status_code() == &response::StatusCode::UNKNOWN {
            match (request.get_method(), request.get_path()) {
                (Method::GET, "/admin") => {
                    if basic_auth_validate(request, &self.config.username, &self.config.password) {
                        let file_name = "admin.html";
                        response.set_status_code(response::StatusCode::STATUS200);
                        let contents =
                            fs::read_to_string(root_path.to_string() + file_name).unwrap();
                        response.set_body(contents);
                    } else {
                        let file_name = "401.html";
                        response.add_header(
                            "WWW-Authenticate".to_string(),
                            "Basic realm=\"WallyWorld\"".to_string(),
                        );
                        response.set_status_code(response::StatusCode::STATUS401);
                        let contents =
                            fs::read_to_string(root_path.to_string() + file_name).unwrap();
                        response.set_body(contents);
                    }
                }
                (_, _) => (),
            }
        }
    }

    fn handle_custom_routes(&self, request: &Request, response: &mut response::Response) {
        let root_path = &self.config.root_path;

        if response.get_status_code() == &response::StatusCode::UNKNOWN {
            match (request.get_method(), request.get_path()) {
                (self::Method::GET, "/sleep") => {
                    let file_name = "hello.html";
                    thread::sleep(Duration::from_secs(
                        request
                            .get_param("time")
                            .unwrap_or(&"5".to_string())
                            .parse()
                            .unwrap_or(0), //5
                    ));
                    let contents = fs::read_to_string(root_path.to_string() + file_name).unwrap();
                    response.set_status_code(response::StatusCode::STATUS200);
                    response.set_body(contents);
                }
                (_, _) => (),
            };
        }
    }

    fn handle_everything_else(&self, request: &Request, response: &mut response::Response) {
        let root_path = &self.config.root_path;

        if response.get_status_code() == &response::StatusCode::UNKNOWN {
            let (status_code, file_name) = match (request.get_method(), request.get_path()) {
                (Method::GET, "/") => {
                    if fs::metadata(root_path.to_string() + self.config.index.as_str()).is_ok() {
                        (response::StatusCode::STATUS200, self.config.index.as_str())
                    } else {
                        (response::StatusCode::STATUS404, "404.html")
                    }
                }
                (self::Method::GET, _) => {
                    if fs::metadata(root_path.to_string() + request.get_path()).is_ok() {
                        (response::StatusCode::STATUS200, request.get_path())
                    } else {
                        (response::StatusCode::STATUS404, "404.html")
                    }
                }

                (_, _) => (response::StatusCode::STATUS501, "501.html"),
            };

            let contents = fs::read_to_string(root_path.to_string() + file_name).unwrap();
            response.set_status_code(status_code);
            response.set_body(contents);
        }
    }
}
