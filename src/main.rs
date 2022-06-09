mod config;
mod models;

use crate::models::Status;
use actix_web::{HttpServer, App, web, Responder, HttpResponse};
use std::io;aa69976b5eb0bc59a3de44d4bc67fdbe9d3ff84a
use dotenv::dotenv;

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

    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();

    println!("\nStarting server at {}:{}", config.server.host, config.server.port);

    HttpServer::new(|| {

        App::new()
            .route("/", web::get().to(status))

    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
