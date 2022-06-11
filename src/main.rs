mod config;
mod db;
mod modules;
mod schema;

use actix_web::{HttpServer, App};
use std::io;

use crate::modules::todos::controller::todo_controller::*;
use config::todo_config;

#[macro_use]
extern crate diesel;
extern crate dotenv;

#[actix_rt::main]
async fn main() -> io::Result<()> {

    static HOST: &str = "127.0.0.1";
    static PORT: &str = "7000";

    println!("\nStarting server at {}:{}", HOST, PORT);

    HttpServer::new(move | | {

        App::new()
            .configure(todo_config)
            .service(status)

            // .service(factory)            

    })
    .bind(format!("{}:{}", HOST, PORT))?
    .run()
    .await
}

