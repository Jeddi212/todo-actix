mod controller;
mod models;
mod repository;
mod schema;

use actix_web::{HttpServer, App};
use std::io;

use crate::controller::*;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

#[macro_use]
extern crate diesel;
extern crate dotenv;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

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

    })
    .bind(format!("{}:{}", HOST, PORT))?
    .run()
    .await
}
