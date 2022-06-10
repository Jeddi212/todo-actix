mod modules;
mod schema;
mod db;

use actix_web::{HttpServer, App};
use std::io;

// use crate::controller::*;
use crate::modules::todos::controller::todo_controller::*;

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
            .service(status)
            .service(get_todo)
            .service(get_todo_by_id)
            .service(post_todo)
            .service(put_todo)
            .service(delete_todo)

    })
    .bind(format!("{}:{}", HOST, PORT))?
    .run()
    .await
}
