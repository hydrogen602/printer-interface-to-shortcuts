mod data_defs;
mod filaments;
mod job_checker;
mod remote;
mod traits;
mod utils;

use actix_web::{delete, get, post, web, App, HttpServer};
use dotenv::dotenv;
use filaments::Filament;
use log::{info, LevelFilter};
use serde::Deserialize;
use simple_logger::SimpleLogger;
use std::sync::Arc;

use traits::printer_trait::Printer;
use utils::http_errors::AnyhowHTTPError;
use utils::logging_util::LoggableResult;
use utils::time_utils;

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

#[get("/job")]
async fn job_status(
    printer: web::Data<dyn Printer>,
    req: actix_web::HttpRequest,
    info: web::Query<Opts>,
) -> Result<String, AnyhowHTTPError> {
    let api_key = utils::get_api_key(&req)?;
    let job_state = printer.job_state(api_key).await.log_error()?;
    let percent = (job_state.progress.completion.unwrap_or(0.)).round() as i32;

    if let Target::HttpSwitch = info.target {
        return Ok(if percent == 100 {
            "0".to_string()
        } else {
            "1".to_string()
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
            (100, _, Some(time_taken), Some(file_name)) => {
                format!(
                    "Finished printing {}. Printing took {}",
                    file_name, time_taken
                )
            }
            (100, _, Some(time_taken), None) => {
                format!("Finished printing. Printing took {}", time_taken)
            }
            (100, _, None, Some(file_name)) => {
                format!(
                    "Finished printing {}. Printing took an unknown amount of time",
                    file_name
                )
            }
            (percent, Some(time_left), _, Some(file_name)) => {
                format!(
                    "Currently printing {}, which is {}% complete. Printing is expected to finish in {}",
                    file_name, percent, time_left,
                )
            }
            (percent, Some(time_left), _, None) => {
                format!(
                    "Currently printing, which is {}% complete. Printing is expected to finish in {}",
                    percent, time_left,
                )
            }
            (percent, None, _, Some(file_name)) => {
                format!(
                    "Currently printing {}, which is {}% complete",
                    file_name, percent,
                )
            }
            (percent, None, _, None) => {
                format!("Currently printing, which is {}% complete", percent)
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
    req: actix_web::HttpRequest,
    info: web::Query<FilamentOpts>,
) -> Result<String, AnyhowHTTPError> {
    let api_key = utils::get_api_key(&req)?;

    printer
        .retract_filament(api_key, info.filament)
        .await
        .log_error()?;
    Ok("Removing filament".to_string())
}

#[post("/filament")]
async fn feed_filament(
    printer: web::Data<dyn Printer>,
    req: actix_web::HttpRequest,
    info: web::Query<FilamentOpts>,
) -> Result<String, AnyhowHTTPError> {
    let api_key = utils::get_api_key(&req)?;

    printer
        .feed_filament(api_key, info.filament)
        .await
        .log_error()?;
    Ok("Feeding filament".to_string())
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

    let printer_clone = printer.clone();
    let _print_finish_notify = tokio::spawn(async move {
        job_checker::job_checker(
            printer_clone,
            remote::notify_homebridge::NotifyHomebridge::new(client),
            &read_key,
        )
        .await
        .log_error()
    });

    info!("Starting server");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(printer.clone()))
            .service(job_status)
            .service(cancel_job)
            .service(remove_filament)
            .service(feed_filament)
    })
    .bind(("0.0.0.0", 5001))?
    .run()
    .await
}

// #[tokio::main]
// async fn main() {
//     let printer = printer_service::PrinterService::new();
//     // let response = printer.version().await.unwrap();
//     // println!("{}", response);
//     let state = printer.printer_state().await.unwrap();
//     println!("{:?}", state);

//     let job = printer.job_state().await.unwrap();
//     println!("{:?}", job);

//     // // printer.home_all().await.unwrap();

//     // printer.retract_filament().await.unwrap();
//     // printer.remove_filament().await.unwrap();
//     // printer.feed_filament().await.unwrap();
//     // printer.cool_down().await.unwrap();

//     // let x = printer.printer_state().await.unwrap();
//     // println!("{:?}", x);
// }
