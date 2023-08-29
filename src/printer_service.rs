#![allow(dead_code)]
use crate::data_defs::printer_move::PrinterMove;
use crate::data_defs::printer_tool::{Targets, Tool};
use crate::{data_defs::printer_state::PrinterState, printer_trait::Printer};
use anyhow::ensure;
use reqwest::header::{self, HeaderMap};
use std::env;

fn get_api_key() -> String {
    env::var("API_KEY").expect("API_KEY not set")
}

fn get_default_headers() -> HeaderMap {
    let mut h = header::HeaderMap::new();
    h.append("X-Api-Key", get_api_key().parse().unwrap());
    h
}

pub struct PrinterService {
    client: reqwest::Client,
}

impl PrinterService {
    const PREFIX: &'static str = "http://octoprint.local/api";

    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .default_headers(get_default_headers())
            .build()
            .unwrap();
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

    async fn get<T>(&self, endpoint: &str) -> anyhow::Result<T>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        let resp = self
            .client
            .get(format!("{}/{}", Self::PREFIX, endpoint))
            .send()
            .await?
            .json()
            .await?;
        Ok(resp)
    }

    async fn post_no_response<U>(&self, endpoint: &str, payload: U) -> anyhow::Result<()>
    where
        U: serde::Serialize,
    {
        let resp = self
            .client
            .post(format!("{}/{}", Self::PREFIX, endpoint))
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
    async fn home_all(&self) -> anyhow::Result<()> {
        self.post_no_response("printer/printhead", PrinterMove::home_all())
            .await?;
        Ok(())
    }

    async fn move_print_head_high(&self) -> anyhow::Result<()> {
        self.post_no_response(
            "printer/printhead",
            PrinterMove::Move {
                x: None,
                y: None,
                z: Some(200.0),
                absolute: Some(true),
                speed: None,
            },
        )
        .await?;
        Ok(())
    }

    async fn hot_end_for_pla(&self) -> anyhow::Result<()> {
        self.post_no_response(
            "printer/tool",
            Tool::Target {
                targets: Targets { tool0: 210 },
            },
        )
        .await?;
        Ok(())
    }

    async fn _cool_down(&self) -> anyhow::Result<()> {
        self.post_no_response(
            "printer/tool",
            Tool::Target {
                targets: Targets { tool0: 0 },
            },
        )
        .await?;
        Ok(())
    }

    async fn _retract_filament(&self) -> anyhow::Result<()> {
        self.post_no_response(
            "printer/tool",
            Tool::Extrude {
                amount: -450.0,
                speed: Some(250.),
            },
        )
        .await?;

        Ok(())
    }

    async fn _feed_filament(&self) -> anyhow::Result<()> {
        self.post_no_response(
            "printer/tool",
            Tool::Extrude {
                amount: 300.0,
                speed: Some(250.),
            },
        )
        .await?;
        self.post_no_response(
            "printer/tool",
            Tool::Extrude {
                amount: 100.0,
                speed: Some(80.),
            },
        )
        .await?;

        self.cool_down().await?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl Printer for PrinterService {
    async fn printer_state(&self) -> anyhow::Result<PrinterState> {
        self.get("printer").await
    }

    async fn prepare_remove_filament(&self) -> anyhow::Result<()> {
        let state = self.printer_state().await?;
        ensure!(
            state.state.flags.operational == true,
            "Printer not operational"
        );

        self.move_print_head_high().await?;
        self.hot_end_for_pla().await
    }

    async fn retract_filament(&self) -> anyhow::Result<()> {
        let state = self.printer_state().await?;
        ensure!(
            state.state.flags.operational == true,
            "Printer not operational"
        );
        // TODO: put temp config into config file
        ensure!(
            state.temperature.tool0.actual > 200.,
            "Hot end not hot enough"
        );

        self._retract_filament().await
    }

    async fn feed_filament(&self) -> anyhow::Result<()> {
        let state = self.printer_state().await?;
        ensure!(
            state.state.flags.operational == true,
            "Printer not operational"
        );
        // TODO: put temp config into config file
        ensure!(
            state.temperature.tool0.actual > 200.,
            "Hot end not hot enough"
        );

        self._feed_filament().await
    }

    async fn cool_down(&self) -> anyhow::Result<()> {
        let state = self.printer_state().await?;
        ensure!(
            state.state.flags.operational == true,
            "Printer not operational"
        );

        self._cool_down().await
    }

    async fn job_state(&self) -> anyhow::Result<crate::data_defs::printer_job_state::JobState> {
        self.get("job").await
    }
}
