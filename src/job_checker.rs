use std::sync::Arc;

use log::info;

use crate::traits::{notify_trait::Notifier, printer_trait::Printer};

pub async fn job_checker(
    printer_service: Arc<dyn Printer>,
    notifier: impl Notifier,
    api_read_key: &str,
) -> anyhow::Result<()> {
    loop {
        let status = printer_service.printer_state(api_read_key).await?;

        if status.state.flags.printing {
            info!("Print job started");
            wait_till_complete(printer_service.as_ref(), api_read_key).await?;
            info!("Print job ended");
            notifier.notify().await?;
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }
}

async fn wait_till_complete(
    printer_service: &dyn Printer,
    api_read_key: &str,
) -> anyhow::Result<()> {
    loop {
        let status = printer_service.printer_state(api_read_key).await?;

        if !status.state.flags.printing {
            break;
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }

    Ok(())
}
