mod data_defs;
mod printer_service;
mod printer_trait;

use printer_trait::Printer;

// use actix_web::{get, web, App, HttpServer, Responder};

// #[get("/")]
// async fn greet() -> impl Responder {
//     format!("Hello World!")
// }

// #[actix_web::main] // or #[tokio::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| App::new().service(greet))
//         .bind(("127.0.0.1", 8080))?
//         .run()
//         .await
// }

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
