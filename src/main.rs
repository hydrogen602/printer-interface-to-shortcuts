mod data_defs;
mod http_errors;
mod interface_data_defs;
mod printer_service;
mod printer_trait;
mod time_utils;

use actix_web::{get, web, App, HttpServer};
use std::sync::Arc;

use http_errors::AnyhowInternalServerError;
use printer_trait::Printer;

#[get("/job")]
async fn job_status(
    printer: web::Data<dyn Printer>,
    req: actix_web::HttpRequest,
) -> Result<String, AnyhowInternalServerError> {
    let api_key = req
        .headers()
        .get("X-Api-Key")
        .ok_or_else(|| {
            AnyhowInternalServerError(anyhow::anyhow!("X-Api-Key header not found in request"))
        })?
        .to_str()
        .map_err(anyhow::Error::from)?;

    let job_state = printer.job_state(api_key).await?;

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

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let printer: Arc<dyn Printer> = Arc::new(printer_service::PrinterService::new());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(printer.clone()))
            .service(job_status)
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
