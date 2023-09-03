use anyhow::ensure;
use reqwest::Client;
use serde::Deserialize;

use crate::traits::notify_trait::Notifier;

#[derive(Deserialize, Debug)]
pub struct NotifyResponse {
    success: bool,
}

/// this pings the homebridge plugin https://www.npmjs.com/package/homebridge-http-doorbell-v3
pub struct NotifyHomebridge {
    pub url: String,
    pub web_client: reqwest::Client,
}

impl NotifyHomebridge {
    pub fn new(web_client: Client) -> Self {
        Self {
            url: "http://192.168.1.240:9091/printjob".to_string(),
            web_client,
        }
    }
}

#[async_trait::async_trait]
impl Notifier for NotifyHomebridge {
    async fn notify(&self) -> anyhow::Result<()> {
        let resp: NotifyResponse = self
            .web_client
            .get(&self.url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        ensure!(resp.success, "Failed to notify homebridge");
        Ok(())
    }
}
