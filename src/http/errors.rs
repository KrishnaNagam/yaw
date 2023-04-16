use super::{response::{Response, self}, ParseError, headers};

pub enum ClientError {
    BadRequest,
    Unauthorized,
    NotFound,
    MethodNotAllowed,
}

pub enum ServerError {
    InternalServerError,
    MethodNotImplemented,
    BadGateway,
    ServiceUnavailable,
}

pub enum HttpError {
    ClientError(ClientError),
    ServerError(ServerError)
}

impl ClientError {
    pub fn to_response(self) -> Response {
        match self {
            ClientError::BadRequest => {
                let mut response = Response::new();
                response.set_status_code(response::Status::InvalidRequest);
                response
            },
            ClientError::Unauthorized => {
                let mut response = Response::new();
                response.add_header(
                    headers::WWW_AUTHENTICATE,
                    "Basic realm=\"WallyWorld\"",
                );
                response.set_status_code(response::Status::Unauthorized);
                response
            },
            ClientError::MethodNotAllowed => {
                let mut response = Response::new();
                response.set_status_code(response::Status::MethodNotAllowed);
                response
            },
            ClientError::NotFound => {
                let mut response = Response::new();
                response.set_status_code(response::Status::NotFound);
                response
            },
        }
    }
}

impl ServerError {
    pub fn to_response(self) -> Response {
        match self {
            ServerError::InternalServerError => {
                let mut response = Response::new();
                response.set_status_code(response::Status::InternalServerError);
                response
            },
            ServerError::MethodNotImplemented => {
                let mut response = Response::new();
                response.set_status_code(response::Status::NotImplemented);
                response
            },
            ServerError::BadGateway => {
                let mut response = Response::new();
                response.set_status_code(response::Status::BadGateway);
                response
            },
            Self::ServiceUnavailable=> {
                let mut response = Response::new();
                response.set_status_code(response::Status::ServiceUnavailable);
                response
            },
        }
    }
}

impl HttpError {
    pub fn to_response(self) -> Response {
        match self {
            Self::ClientError(e) => e.to_response(),
            Self::ServerError(e) => e.to_response()
        }
    }
}

impl From<ParseError> for HttpError {
    fn from(_: ParseError) -> Self {
        HttpError::ClientError(ClientError::BadRequest)
    }
}