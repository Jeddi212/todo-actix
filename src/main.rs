mod controller;
mod models;

use actix_web::{HttpServer, App};
use std::io;

use crate::controller::*;

#[actix_rt::main]
async fn main() -> io::Result<()> {

    static HOST: &str = "127.0.0.1";
    static PORT: &str = "7000";

    println!("\nStarting server at {}:{}", HOST, PORT);

    HttpServer::new(move | | {

        App::new()
            .service(status)
            .service(get_todo)
            .service(post_todo)
            .service(put_todo)

    })
    .bind(format!("{}:{}", HOST, PORT))?
    .run()
    .await
}
