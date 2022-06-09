mod models;

use crate::models::Status;
use actix_web::{HttpServer, App, web, Responder, HttpResponse};
use std::io;

async fn status() -> impl Responder {
    HttpResponse::Ok()
        .json(
            Status { 
                status: "UP".to_string()
            }
        )
}

#[actix_rt::main]
async fn main() -> io::Result<()> {

    println!("\nStarting server at localhost:7000");

    HttpServer::new(|| {

        App::new()
            .route("/", web::get().to(status))

    })
    .bind("127.0.0.1:7000")?
    .run()
    .await
}
