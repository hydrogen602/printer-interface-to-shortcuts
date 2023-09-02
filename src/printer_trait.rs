use crate::data_defs::{printer_job_state::JobState, printer_state::PrinterState};

#[async_trait::async_trait]
pub trait Printer: Send + Sync {
    async fn printer_state(&self, api_key: &str) -> anyhow::Result<PrinterState>;
    async fn prepare_remove_filament(&self, api_key: &str) -> anyhow::Result<()>;
    async fn retract_filament(&self, api_key: &str) -> anyhow::Result<()>;
    async fn feed_filament(&self, api_key: &str) -> anyhow::Result<()>;
    async fn cool_down(&self, api_key: &str) -> anyhow::Result<()>;
    async fn job_state(&self, api_key: &str) -> anyhow::Result<JobState>;
    async fn cancel_job(&self, api_key: &str) -> anyhow::Result<()>;
}
