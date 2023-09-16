use thiserror::Error;

#[derive(Error, Debug)] //AnyhowInternalServerError

// these are all going to be send out, so turn errors into strings
pub enum AnyhowHTTPError {
    #[error("Internal Server Error 500: {0}")]
    InternalServerError500(String),
    #[error("Unauthorized 401: {0}")]
    Unauthorized401(String),
    #[error("Conflict 409: {0}")]
    Conflict409(String),
    #[error("HTTPError: {code} {message}")]
    AnyHTTPError { code: u16, message: String },
}

impl actix_web::error::ResponseError for AnyhowHTTPError {
    fn error_response(&self) -> actix_web::HttpResponse {
        match self {
            Self::InternalServerError500(e) => {
                actix_web::HttpResponse::InternalServerError().body(e.clone())
            }
            Self::Conflict409(e) => actix_web::HttpResponse::Conflict().body(e.clone()),
            Self::Unauthorized401(e) => actix_web::HttpResponse::Unauthorized().body(e.clone()),
            Self::AnyHTTPError { code, message } => actix_web::HttpResponse::build(
                actix_web::http::StatusCode::from_u16(*code).unwrap(),
            )
            .body(message.clone()),
        }
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }
}

impl From<anyhow::Error> for AnyhowHTTPError {
    fn from(e: anyhow::Error) -> Self {
        match e.downcast::<reqwest::Error>() {
            Ok(e) => {
                if let Some(status) = e.status() {
                    return Self::AnyHTTPError {
                        code: status.as_u16(),
                        message: e.to_string(),
                    };
                }
                Self::InternalServerError500(e.to_string())
            }
            Err(e) => Self::InternalServerError500(e.to_string()),
        }
        // Self::InternalServerError500(e.to_string())
    }
}
