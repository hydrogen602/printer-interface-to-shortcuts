use std::fmt::Display;

pub trait LoggableResult<T, E> {
    fn log_warn(self) -> Self;
    fn log_error(self) -> Self;
}

impl<T, E> LoggableResult<T, E> for Result<T, E>
where
    E: Display,
{
    fn log_warn(self) -> Self {
        if let Err(e) = &self {
            log::warn!("{}", e);
        }
        self
    }

    fn log_error(self) -> Self {
        if let Err(e) = &self {
            log::error!("{}", e);
        }
        self
    }
}
