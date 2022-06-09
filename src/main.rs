mod config;
mod handlers;
mod models;

use actix_web::{HttpServer, App, web};
use std::io;
use dotenv::dotenv;
use tokio_postgres::NoTls;
use deadpool_postgres::Runtime;
use crate::handlers::*;

#[actix_rt::main]
async fn main() -> io::Result<()> {

    dotenv().ok();

    let mut config: config::Config;
    let config = crate::config::Config::from_env(&mut config);

    let pool = config.pg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();

    println!("\nStarting server at {}:{}", config.server.host, config.server.port);

    HttpServer::new(move | | {

        App::new()
            .app_data(pool.clone())
            .route("/", web::get().to(status))

    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
