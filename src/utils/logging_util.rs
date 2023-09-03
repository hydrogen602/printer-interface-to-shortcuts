use std::fmt::Display;

pub trait LoggableResult<T, E> {
    fn log_warn(self) -> Self;
    fn log_error(self) -> Self;
    fn log_error_and_panic(self) -> T;
    fn log_error_and_panic_with_msg(self, msg: &str) -> T;
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

    fn log_error_and_panic(self) -> T {
        match self {
            Ok(t) => t,
            Err(e) => {
                log::error!("{}", e);
                panic!("{}", e);
            }
        }
    }

    fn log_error_and_panic_with_msg(self, msg: &str) -> T {
        match self {
            Ok(t) => t,
            Err(e) => {
                log::error!("{}: {}", msg, e);
                panic!("{}: {}", msg, e);
            }
        }
    }
}
