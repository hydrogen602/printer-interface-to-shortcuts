use anyhow::anyhow;
use reqwest::Response;
use serde::de::DeserializeOwned;

#[async_trait::async_trait]
pub trait LogInvalidJson<T> {
    async fn json_log_if_invalid(self) -> anyhow::Result<T>;
}

#[async_trait::async_trait]
impl<T> LogInvalidJson<T> for Response
where
    T: DeserializeOwned,
{
    async fn json_log_if_invalid(self) -> anyhow::Result<T> {
        let contents = self.text().await?;
        match serde_json::from_str(&contents) {
            Ok(json) => Ok(json),
            Err(e) => {
                log::error!("Invalid JSON response: {}", e);
                Err(anyhow!(e))
            }
        }
    }
}
