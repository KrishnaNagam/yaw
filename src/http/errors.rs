use super::response::{Response, self};

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
                response.set_status_code(response::StatusCode::STATUS400);
                response
            },
            ClientError::Unauthorized => {
                let mut response = Response::new();
                response.set_status_code(response::StatusCode::STATUS401);
                response
            },
            ClientError::MethodNotAllowed => {
                let mut response = Response::new();
                response.set_status_code(response::StatusCode::STATUS405);
                response
            },
            ClientError::NotFound => {
                let mut response = Response::new();
                response.set_status_code(response::StatusCode::STATUS404);
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
                response.set_status_code(response::StatusCode::STATUS500);
                response
            },
            ServerError::MethodNotImplemented => {
                let mut response = Response::new();
                response.set_status_code(response::StatusCode::STATUS501);
                response
            },
            ServerError::BadGateway => {
                let mut response = Response::new();
                response.set_status_code(response::StatusCode::STATUS502);
                response
            },
            Self::ServiceUnavailable=> {
                let mut response = Response::new();
                response.set_status_code(response::StatusCode::STATUS503);
                response
            },
        }
    }
}

// impl HttpError {
//     pub fn to_response(self) -> Response {
//         match self {
//             Self::ClientError(e) => e.to_response(),
//             Self::ServerError(e) => e.to_response()
//         }
//     }
// }