#![allow(dead_code)]

use anyhow::ensure;
use log::debug;
use reqwest::header::{self, HeaderMap};
use reqwest::Client;

use super::error_util::LogInvalidJson;
use crate::data_defs::printer_job_action::JobAction;
use crate::data_defs::printer_move::PrinterMove;
use crate::data_defs::printer_tool::{Targets, Tool};
use crate::filaments::{Filament, HotEndTemperature};
use crate::{data_defs::printer_state::PrinterState, traits::printer_trait::Printer};

fn get_default_headers(api_key: &str) -> HeaderMap {
    let mut h = header::HeaderMap::new();
    h.append("X-Api-Key", api_key.parse().unwrap());
    h
}

pub struct PrinterService {
    client: reqwest::Client,
}

impl PrinterService {
    const PREFIX: &'static str = "http://192.168.1.113/api";

    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn version(&self) -> anyhow::Result<String> {
        let resp = self
            .client
            .get(format!("{}/version", PrinterService::PREFIX))
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;
        Ok(resp)
    }

    async fn get<T>(&self, endpoint: &str, api_key: &str) -> anyhow::Result<T>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        let resp = self
            .client
            .get(format!("{}/{}", Self::PREFIX, endpoint))
            .headers(get_default_headers(api_key))
            .send()
            .await?
            .error_for_status()?
            .json_log_if_invalid()
            .await?;
        Ok(resp)
    }

    async fn post_no_response<U>(
        &self,
        endpoint: &str,
        payload: U,
        api_key: &str,
    ) -> anyhow::Result<()>
    where
        U: serde::Serialize,
    {
        let resp = self
            .client
            .post(format!("{}/{}", Self::PREFIX, endpoint))
            .headers(get_default_headers(api_key))
            .json(&payload)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;
        debug!("{}", resp);
        Ok(())
    }
}

impl PrinterService {
    async fn home_all(&self, api_key: &str) -> anyhow::Result<()> {
        self.post_no_response("printer/printhead", PrinterMove::home_all(), api_key)
            .await?;
        Ok(())
    }

    async fn move_print_head_high(&self, api_key: &str) -> anyhow::Result<()> {
        self.post_no_response(
            "printer/printhead",
            PrinterMove::Move {
                x: None,
                y: None,
                z: Some(200.0),
                absolute: Some(true),
                speed: None,
            },
            api_key,
        )
        .await?;
        Ok(())
    }

    async fn hot_end(&self, api_key: &str, temperature: HotEndTemperature) -> anyhow::Result<()> {
        self.post_no_response(
            "printer/tool",
            Tool::Target {
                targets: Targets {
                    tool0: temperature.into(),
                },
            },
            api_key,
        )
        .await?;
        Ok(())
    }

    async fn _cool_down(&self, api_key: &str) -> anyhow::Result<()> {
        self.post_no_response(
            "printer/tool",
            Tool::Target {
                targets: Targets { tool0: 0 },
            },
            api_key,
        )
        .await?;
        Ok(())
    }

    /// This will block for a long time (10 min ish)
    /// waits until hot-end is within 5 degrees of target
    /// Polls every 10 seconds
    async fn _wait_for_temperature(
        &self,
        api_key: &str,
        target: HotEndTemperature,
    ) -> anyhow::Result<()> {
        loop {
            let state = self.printer_state(api_key).await?;
            if target.within_5_degrees_of(state.temperature.tool0.actual) {
                break;
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        }
        Ok(())
    }

    async fn _retract_filament(&self, api_key: &str) -> anyhow::Result<()> {
        self.post_no_response(
            "printer/tool",
            Tool::Extrude {
                amount: -450.0,
                speed: Some(250.),
            },
            api_key,
        )
        .await?;

        Ok(())
    }

    async fn _feed_filament(&self, api_key: &str) -> anyhow::Result<()> {
        // self.post_no_response(
        //     "printer/tool",
        //     Tool::Extrude {
        //         amount: 300.0,
        //         speed: Some(250.),
        //     },
        //     api_key,
        // )
        // .await?;
        self.post_no_response(
            "printer/tool",
            Tool::Extrude {
                amount: 500.0,
                speed: Some(80.),
            },
            api_key,
        )
        .await?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl Printer for PrinterService {
    async fn printer_state(&self, api_key: &str) -> anyhow::Result<PrinterState> {
        self.get("printer", api_key).await
    }

    async fn retract_filament(&self, api_key: &str, filament: Filament) -> anyhow::Result<()> {
        let state = self.printer_state(api_key).await?;
        ensure!(state.state.flags.operational, "Printer not operational");

        self.hot_end(api_key, filament.into()).await?;
        self.home_all(api_key).await?;
        self.move_print_head_high(api_key).await?;

        self._wait_for_temperature(api_key, filament.into()).await?;

        self._retract_filament(api_key).await
    }

    async fn feed_filament(&self, api_key: &str, filament: Filament) -> anyhow::Result<()> {
        let state = self.printer_state(api_key).await?;
        ensure!(state.state.flags.operational, "Printer not operational");

        self.hot_end(api_key, filament.into()).await?;
        self.home_all(api_key).await?;
        self.move_print_head_high(api_key).await?;

        self._wait_for_temperature(api_key, filament.into()).await?;

        self._feed_filament(api_key).await
    }

    async fn cool_down(&self, api_key: &str) -> anyhow::Result<()> {
        let state = self.printer_state(api_key).await?;
        // TODO: replace with something returning 409 (Conflict)
        ensure!(state.state.flags.operational, "Printer not operational");

        self._cool_down(api_key).await
    }

    async fn job_state(
        &self,
        api_key: &str,
    ) -> anyhow::Result<crate::data_defs::printer_job_state::JobState> {
        self.get("job", api_key).await
    }

    async fn cancel_job(&self, api_key: &str) -> anyhow::Result<()> {
        self.post_no_response("job", JobAction::Cancel, api_key)
            .await
    }
}
