pub mod http_errors;
pub mod logging_util;
pub mod retry_on_fail;
pub mod time_utils;

use http_errors::AnyhowHTTPError;
use logging_util::LoggableResult;

pub fn get_api_key(req: &actix_web::HttpRequest) -> Result<&str, AnyhowHTTPError> {
    Ok(req
        .headers()
        .get("X-Api-Key")
        .ok_or_else(|| {
            AnyhowHTTPError::Unauthorized401("X-Api-Key header not found in request".to_string())
        })
        .log_warn()?
        .to_str()
        .map_err(anyhow::Error::from)?)
}
