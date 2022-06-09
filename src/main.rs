// mod config;
mod controller;
mod models;

use actix_web::{HttpServer, App};
use std::io;
// use dotenv::dotenv;
// use tokio_postgres::NoTls;
// use deadpool_postgres::Runtime;
use crate::controller::*;

#[actix_rt::main]
async fn main() -> io::Result<()> {

    // dotenv().ok();

    // let mut config: config::Config;
    // let config = crate::config::Config::from_env(&mut config);

    // let pool = config.pg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();

    static HOST: &str = "127.0.0.1";
    static PORT: &str = "7000";

    println!("\nStarting server at {}:{}", HOST, PORT);

    HttpServer::new(move | | {

        App::new()
            .service(status)
            .service(get_todo)
            .service(get_todos)

    })
    .bind(format!("{}:{}", HOST, PORT))?
    .run()
    .await
}
