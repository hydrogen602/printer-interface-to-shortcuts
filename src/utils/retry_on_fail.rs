use std::{fmt::Display, future::Future};

pub async fn retry_on_fail<F, T, E, R>(f: F) -> T
where
    F: Fn() -> R,
    E: Display,
    R: Future<Output = Result<T, E>>,
{
    loop {
        match f().await {
            Ok(t) => return t,
            Err(e) => log::error!("Error: {}\nretrying...", e),
        }
    }
}
