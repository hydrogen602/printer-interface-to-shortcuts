mod data_defs;
mod printer_trait;
mod remote;
mod utils;

use actix_web::{delete, get, web, App, HttpServer};
use log::{info, LevelFilter};
use simple_logger::SimpleLogger;
use std::sync::Arc;

use printer_trait::Printer;
use utils::http_errors::AnyhowHTTPError;
use utils::logging_util::LoggableResult;
use utils::time_utils;

#[get("/job")]
async fn job_status(
    printer: web::Data<dyn Printer>,
    req: actix_web::HttpRequest,
) -> Result<String, AnyhowHTTPError> {
    let api_key = utils::get_api_key(&req)?;
    let job_state = printer.job_state(api_key).await.log_error()?;

    let percent = (job_state.progress.completion).round() as i32;
    let seconds_left = job_state.progress.print_time_left;
    let time_left = time_utils::Time::from_seconds(seconds_left)
        .unwrap()
        .to_human_readable_briefly();

    if percent == 100 {
        let time_taken = time_utils::Time::from_seconds(job_state.progress.print_time)
            .unwrap()
            .to_human_readable_briefly();
        return Ok(format!(
            "Finished printing {}. Printing took {}",
            job_state.job.file.name, time_taken
        ));
    }

    Ok(format!(
        "Currently printing {}, which is {}% complete. Printing is expected to finish in {}",
        job_state.job.file.name, percent, time_left
    ))
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

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    let printer: Arc<dyn Printer> = Arc::new(remote::printer_service::PrinterService::new());

    info!("Starting server");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(printer.clone()))
            .service(job_status)
            .service(cancel_job)
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
