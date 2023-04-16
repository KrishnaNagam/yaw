use std::{fs, sync::Arc};

use crate::{
    config::Config,
    http::{
        auth::basic_auth_validate,
        errors::{ClientError, HttpError, ServerError},
        request::{Method, Request},
        response::{self, Response},
    },
};

pub struct RequestProcessor {
    config: Arc<Config>,
}

impl RequestProcessor {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config: config,
        }
    }

    pub fn process(&self, request: Request) -> Result<response::Response, HttpError> {
        let mut response = response::Response::new();

        self.handle_authentication(&request, &mut response)?;
        self.handle_routes(&request, &mut response)?;

        Ok(response)
    }

    fn handle_authentication(
        &self,
        request: &Request,
        response: &mut response::Response,
    ) -> Result<(), HttpError> {
        match (request.get_method(), request.get_path()) {
            (Method::GET, "/admin") => {
                if !basic_auth_validate(request, &self.config.username, &self.config.password) {
                    return Err(HttpError::ClientError(ClientError::Unauthorized))
                }
            }
            (_, _) => {}
        }
        Ok(())
    }

    fn handle_routes(
        &self,
        request: &Request,
        response: &mut response::Response,
    ) -> Result<(), HttpError> {
        if response.get_status_code() == &response::Status::Ok {
            match (request.get_method(), request.get_path()) {
                (self::Method::GET, "/admin") => {
                    let file_name = "admin.html";
                    self.load_content_from_file(response, file_name)?
                }
                (Method::GET, "/") => {
                    let file_name = self.config.index.as_str();
                    self.load_content_from_file(response, file_name)?
                },
                (self::Method::GET, _) => {
                    let file_name = request.get_path();
                    for (path,handler) in &self.config.request_handlers {
                        if path == &request.get_path() {
                            handler(request,response);
                            return Ok(())
                        }
                    }
                    self.load_content_from_file(response, file_name)?;
                },
                (_, _) => return Err(HttpError::ServerError(ServerError::MethodNotImplemented)),
            };
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
