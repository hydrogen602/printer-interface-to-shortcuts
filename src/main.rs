mod data_defs;
mod filaments;
mod job_checker;
mod remote;
mod traits;
mod utils;

use actix_web::{delete, get, post, web, App, HttpServer, Responder};
use anyhow::anyhow;
use dotenv::dotenv;
use filaments::Filament;
use log::{info, LevelFilter};
use serde::Deserialize;
use simple_logger::SimpleLogger;
use std::borrow::BorrowMut;
use std::mem;
use std::sync::Arc;
// use tokio::task::JoinHandle;
use utils::job_running::{run_job, JobStatus, LongRunningJob};

use traits::printer_trait::Printer;
use utils::http_errors::AnyhowHTTPError;
use utils::logging_util::LoggableResult;
use utils::retry_on_fail::retry_on_fail;
use utils::time_utils;

const BUILD_TIME: &str = include!(concat!(env!("OUT_DIR"), "/timestamp.txt"));

#[derive(Deserialize, Debug)]
struct Opts {
    #[serde(default)]
    target: Target,
}

#[derive(Default, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
enum Target {
    #[default]
    Siri,
    HttpSwitch,
}

/// if target == HttpSwitch, then it returns 1 for job active, 0 for job inactive
#[get("/job")]
async fn job_status(
    printer: web::Data<dyn Printer>,
    req: actix_web::HttpRequest,
    info: web::Query<Opts>,
) -> Result<String, AnyhowHTTPError> {
    let api_key = utils::get_api_key(&req)?;
    let job_state = printer.job_state(api_key).await.log_error()?;
    let percent = job_state.progress.completion.map(|c| c.round() as i32);

    if let Target::HttpSwitch = info.target {
        return Ok(match percent {
            // no job
            None => "0".to_string(),
            // job done
            Some(100) => "0".to_string(),
            // job in progress
            Some(_) => "1".to_string(),
        });
    }

    let time_left = job_state.progress.print_time_left.map(|time| {
        time_utils::Time::from_seconds(time)
            .unwrap()
            .to_human_readable_briefly()
    });

    let time_taken = job_state.progress.print_time.map(|time| {
        time_utils::Time::from_seconds(time)
            .unwrap()
            .to_human_readable_briefly()
    });

    Ok(
        match (percent, time_left, time_taken, job_state.job.file.name) {
            (Some(100), _, Some(time_taken), Some(file_name)) => {
                format!(
                    "Finished printing {}. Printing took {}",
                    file_name, time_taken
                )
            }
            (Some(100), _, Some(time_taken), None) => {
                format!("Finished printing. Printing took {}", time_taken)
            }
            (Some(100), _, None, Some(file_name)) => {
                format!(
                    "Finished printing {}. Printing took an unknown amount of time",
                    file_name
                )
            }
            (Some(percent), Some(time_left), _, Some(file_name)) => {
                format!(
                    "Currently printing {}, which is {}% complete. Printing is expected to finish in {}",
                    file_name, percent, time_left,
                )
            }
            (Some(percent), Some(time_left), _, None) => {
                format!(
                    "Currently printing, which is {}% complete. Printing is expected to finish in {}",
                    percent, time_left,
                )
            }
            (Some(percent), None, _, Some(file_name)) => {
                format!(
                    "Currently printing {}, which is {}% complete",
                    file_name, percent,
                )
            }
            (Some(percent), None, _, None) => {
                format!("Currently printing, which is {}% complete", percent)
            }
            (None, _, _, _) => {
                format!("Nothing is currently printing",)
            }
        },
    )
}

#[delete("/job")]
async fn cancel_job(
    printer: web::Data<dyn Printer>,
    req: actix_web::HttpRequest,
) -> Result<String, AnyhowHTTPError> {
    let api_key = utils::get_api_key(&req)?;

    // the printer will return an error if there is no job to cancel (409)
    printer.cancel_job(api_key).await.log_error()?;
    Ok("Cancelling print job".to_string())
}

#[derive(Deserialize, Debug)]
struct FilamentOpts {
    filament: Filament,
}

#[delete("/filament")]
async fn remove_filament(
    printer: web::Data<dyn Printer>,
    long_running_job_tracker: web::Data<tokio::sync::Mutex<LongRunningJob>>,
    req: actix_web::HttpRequest,
    info: web::Query<FilamentOpts>,
) -> Result<String, AnyhowHTTPError> {
    let api_key = utils::get_api_key(&req)?.to_string();

    let mut long_running_job = long_running_job_tracker.lock().await;

    run_job(
        async move {
            printer
                .retract_filament(&api_key, info.filament)
                .await
                .map(|_| "Finished removing filament".to_string())
                .log_error()
        },
        long_running_job.borrow_mut(),
    )?;

    Ok("Job started".to_string())
}

#[post("/filament")]
async fn feed_filament(
    printer: web::Data<dyn Printer>,
    long_running_job_tracker: web::Data<tokio::sync::Mutex<LongRunningJob>>,
    req: actix_web::HttpRequest,
    info: web::Query<FilamentOpts>,
) -> Result<String, AnyhowHTTPError> {
    let api_key = utils::get_api_key(&req)?.to_string();

    let mut long_running_job = long_running_job_tracker.lock().await;

    run_job(
        async move {
            printer
                .feed_filament(&api_key, info.filament)
                .await
                .map(|_| "Finished feeding filament".to_string())
                .log_error()
        },
        long_running_job.borrow_mut(),
    )?;

    Ok("Job started".to_string())
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct ServerInfo {
    build_time: &'static str,
    job_status: JobStatus,
}

#[get("/server-info")]
async fn server_info(
    long_running_job_tracker: web::Data<tokio::sync::Mutex<LongRunningJob>>,
) -> Result<impl Responder, AnyhowHTTPError> {
    let mut long_running_job = long_running_job_tracker.lock().await;
    // let x = long_running_job.job.unwrap().try_into().unwrap();

    let status = match &long_running_job.job {
        None => JobStatus::NoJob,
        Some(job) if job.is_finished() => {
            // we by now verified that the job exists and is finished
            let job_output = mem::take(&mut long_running_job.job)
                .unwrap()
                .await
                .log_error()
                .map_err(|e| anyhow!(e))?;

            job_output.into()
        }
        Some(_) => JobStatus::Running,
    };

    let result = ServerInfo {
        build_time: BUILD_TIME,
        job_status: status,
    };

    Ok(web::Json(result))
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    dotenv().log_error_and_panic_with_msg("Failed to load .env file");

    let read_key: String =
        std::env::var("API_READ_KEY").log_error_and_panic_with_msg("API_READ_KEY not set");

    let client = reqwest::Client::builder().build().unwrap();

    let printer: Arc<dyn Printer> =
        Arc::new(remote::printer_service::PrinterService::new(client.clone()));

    let long_running_job_tracker = Arc::new(tokio::sync::Mutex::new(LongRunningJob { job: None }));

    let printer_clone = printer.clone();
    let client_clone = client.clone();
    let read_key_clone = read_key.clone();

    let job_check = move || {
        let printer_clone2 = printer_clone.clone();
        let client_clone2 = client_clone.clone();
        let read_key_clone2 = read_key_clone.clone();

        async move {
            job_checker::job_checker(
                printer_clone2,
                remote::notify_homebridge::NotifyHomebridge::new(client_clone2),
                &read_key_clone2,
            )
            .await
        }
    };
    let _print_finish_notify = tokio::spawn(retry_on_fail(job_check));

    info!("Starting server with version {}", env!("CARGO_PKG_VERSION"));
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(printer.clone()))
            .app_data(web::Data::from(long_running_job_tracker.clone()))
            .service(job_status)
            .service(cancel_job)
            .service(remove_filament)
            .service(feed_filament)
            .service(server_info)
    })
    .bind(("0.0.0.0", 5001))?
    .run()
    .await
}
