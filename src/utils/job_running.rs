use std::{borrow::BorrowMut, fmt::Display, future::Future};

use tokio::task::JoinHandle;

use crate::data_defs::printer_job_state::Job;

use super::http_errors::AnyhowHTTPError;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum JobStatus {
    NoJob,
    Running,
    Finished(String),
    Error(String),
}

impl<E: ToString> From<Result<String, E>> for JobStatus {
    fn from(result: Result<String, E>) -> Self {
        match result {
            Ok(s) => Self::Finished(s),
            Err(e) => Self::Error(e.to_string()),
        }
    }
}

pub struct LongRunningJob {
    pub job: Option<JoinHandle<anyhow::Result<String>>>,
}

pub fn run_job<T>(task: T, long_running_job: &mut LongRunningJob) -> Result<(), AnyhowHTTPError>
where
    T: Future<Output = anyhow::Result<String>> + Send + 'static,
{
    match long_running_job.job {
        Some(ref job) if job.is_finished() => {}
        Some(_) => {
            return Err(AnyhowHTTPError::Conflict409(
                "Already running a job".to_string(),
            ))
        }
        None => {}
    }

    long_running_job.job = Some(tokio::spawn(task));

    Ok(())
}
