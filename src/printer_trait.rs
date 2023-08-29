use crate::data_defs::{printer_job_state::JobState, printer_state::PrinterState};

#[async_trait::async_trait]
pub trait Printer {
    async fn printer_state(&self) -> anyhow::Result<PrinterState>;
    async fn prepare_remove_filament(&self) -> anyhow::Result<()>;
    async fn retract_filament(&self) -> anyhow::Result<()>;
    async fn feed_filament(&self) -> anyhow::Result<()>;
    async fn cool_down(&self) -> anyhow::Result<()>;
    async fn job_state(&self) -> anyhow::Result<JobState>;
}
