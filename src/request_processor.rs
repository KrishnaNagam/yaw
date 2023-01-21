use std::{fs, sync::Arc, thread, time::Duration};

use crate::{
    config::Config,
    http::{
        auth::basic_auth_validate,
        errors::{ClientError, HttpError, ServerError},
        request::{Method, Request},
        response,
    },
};

pub struct RequestProcessor {
    config: Arc<Config>,
}

impl RequestProcessor {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config: config }
    }

    pub fn process(self, request: Request) -> Result<response::Response, HttpError> {
        let mut response = response::Response::new();

        self.handle_authentication(&request, &mut response)?;
        self.handle_custom_routes(&request, &mut response)?;
        self.handle_everything_else(&request, &mut response)?;

        Ok(response)
    }

    fn handle_authentication(
        &self,
        request: &Request,
        response: &mut response::Response,
    ) -> Result<(), HttpError> {
        let file_name: &str;

        if response.get_status_code() == &response::StatusCode::UNKNOWN {
            match (request.get_method(), request.get_path()) {
                (Method::GET, "/admin") => {
                    if basic_auth_validate(request, &self.config.username, &self.config.password) {
                        file_name = "admin.html";
                        response.set_status_code(response::StatusCode::STATUS200);
                    } else {
                        file_name = "401.html";
                        response.add_header(
                            "WWW-Authenticate".to_string(),
                            "Basic realm=\"WallyWorld\"".to_string(),
                        );
                        response.set_status_code(response::StatusCode::STATUS401);
                    }
                    self.load_content_from_file(response, file_name)?
                }
                (_, _) => {}
            }
        }
        Ok(())
    }

    fn handle_custom_routes(
        &self,
        request: &Request,
        response: &mut response::Response,
    ) -> Result<(), HttpError> {
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
                    response.set_status_code(response::StatusCode::STATUS200);
                    self.load_content_from_file(response, file_name)?
                }
                (_, _) => (),
            };
        }
        Ok(())
    }

    fn handle_everything_else(
        &self,
        request: &Request,
        response: &mut response::Response,
    ) -> Result<(), HttpError> {
        if response.get_status_code() == &response::StatusCode::UNKNOWN {
            let file_name = match (request.get_method(), request.get_path()) {
                (Method::GET, "/") => self.config.index.as_str(),
                (self::Method::GET, _) => request.get_path(),
                (_, _) => return Err(HttpError::ServerError(ServerError::MethodNotImplemented)),
            };
            response.set_status_code(response::StatusCode::STATUS200);
            self.load_content_from_file(response, file_name)?
        }

        Ok(())
    }

    fn load_content_from_file(
        &self,
        response: &mut response::Response,
        file_name: &str,
    ) -> Result<(), HttpError> {
        let root_path = &self.config.root_path;

        match fs::metadata(root_path.to_string() + file_name) {
            Ok(_) => match fs::read_to_string(root_path.to_string() + file_name) {
                Ok(contents) => {
                    response.set_body(contents);
                }
                Err(e) => {
                    print!("{}", e);
                    return Err(HttpError::ServerError(ServerError::InternalServerError));
                }
            },
            Err(file_not_found_err) => {
                print!("{}", file_not_found_err);
                return Err(HttpError::ClientError(ClientError::NotFound));
            }
        };
        Ok(())
    }
}
