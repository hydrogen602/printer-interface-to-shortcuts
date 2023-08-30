#[derive(Debug)] //AnyhowInternalServerError
pub struct AnyhowInternalServerError(pub anyhow::Error);

impl std::fmt::Display for AnyhowInternalServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl actix_web::error::ResponseError for AnyhowInternalServerError {
    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::InternalServerError().json(self.0.to_string())
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }
}

impl From<anyhow::Error> for AnyhowInternalServerError {
    fn from(err: anyhow::Error) -> Self {
        AnyhowInternalServerError(err)
    }
}
