#![allow(dead_code)]
use crate::data_defs::printer_move::PrinterMove;
use crate::data_defs::printer_tool::{Targets, Tool};
use crate::{data_defs::printer_state::PrinterState, printer_trait::Printer};
use anyhow::ensure;
use reqwest::header::{self, HeaderMap};

fn get_default_headers(api_key: &str) -> HeaderMap {
    let mut h = header::HeaderMap::new();
    h.append("X-Api-Key", api_key.parse().unwrap());
    h
}

pub struct PrinterService {
    client: reqwest::Client,
}

impl PrinterService {
    const PREFIX: &'static str = "http://octoprint.local/api";

    pub fn new() -> Self {
        let client = reqwest::Client::builder().build().unwrap();
        Self { client }
    }

    pub async fn version(&self) -> anyhow::Result<String> {
        let resp = self
            .client
            .get("http://octoprint.local/api/version")
            .send()
            .await?
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
            .json()
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
            .text()
            .await?;
        println!("{}", resp);
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

    async fn hot_end_for_pla(&self, api_key: &str) -> anyhow::Result<()> {
        self.post_no_response(
            "printer/tool",
            Tool::Target {
                targets: Targets { tool0: 210 },
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
        self.post_no_response(
            "printer/tool",
            Tool::Extrude {
                amount: 300.0,
                speed: Some(250.),
            },
            api_key,
        )
        .await?;
        self.post_no_response(
            "printer/tool",
            Tool::Extrude {
                amount: 100.0,
                speed: Some(80.),
            },
            api_key,
        )
        .await?;

        self.cool_down(api_key).await?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl Printer for PrinterService {
    async fn printer_state(&self, api_key: &str) -> anyhow::Result<PrinterState> {
        self.get("printer", api_key).await
    }

    async fn prepare_remove_filament(&self, api_key: &str) -> anyhow::Result<()> {
        let state = self.printer_state(api_key).await?;
        ensure!(
            state.state.flags.operational == true,
            "Printer not operational"
        );

        self.move_print_head_high(api_key).await?;
        self.hot_end_for_pla(api_key).await
    }

    async fn retract_filament(&self, api_key: &str) -> anyhow::Result<()> {
        let state = self.printer_state(api_key).await?;
        ensure!(
            state.state.flags.operational == true,
            "Printer not operational"
        );
        // TODO: put temp config into config file
        ensure!(
            state.temperature.tool0.actual > 200.,
            "Hot end not hot enough"
        );

        self._retract_filament(api_key).await
    }

    async fn feed_filament(&self, api_key: &str) -> anyhow::Result<()> {
        let state = self.printer_state(api_key).await?;
        ensure!(
            state.state.flags.operational == true,
            "Printer not operational"
        );
        // TODO: put temp config into config file
        ensure!(
            state.temperature.tool0.actual > 200.,
            "Hot end not hot enough"
        );

        self._feed_filament(api_key).await
    }

    async fn cool_down(&self, api_key: &str) -> anyhow::Result<()> {
        let state = self.printer_state(api_key).await?;
        ensure!(
            state.state.flags.operational == true,
            "Printer not operational"
        );

        self._cool_down(api_key).await
    }

    async fn job_state(
        &self,
        api_key: &str,
    ) -> anyhow::Result<crate::data_defs::printer_job_state::JobState> {
        self.get("job", api_key).await
    }
}
