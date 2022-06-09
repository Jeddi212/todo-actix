use crate::models::*;
use actix_web::{get, post, put, web, HttpResponse, Responder, Result};
use serde::{self, Serialize, Deserialize};

#[get("/")]
pub async fn status() -> impl Responder {
    HttpResponse::Ok()
        .json(
            Status { 
                status: "UP".to_string()
            }
        )
}

#[get("/todo")]
pub async fn get_todo() -> impl Responder {
    HttpResponse::Ok()
        .json(
            TodoList {
                id: 1,
                title: "Todo List 1".to_string()
            }
        )
}

#[post("/todo")]
pub async fn post_todo(info: web::Json<Info>) -> Result<impl Responder> {
    let i = Info {
        username: info.username.to_string()
    };
    Ok(web::Json(i))
}

#[put("/todo/{id}")]
pub async fn put_todo(id: web::Path<String>) -> Result<String> {
    Ok(format!("Your id is {}!", id))
}

#[derive(Serialize, Deserialize)]
pub struct Info {
    username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Params {
    name: String,
    color: String,
}