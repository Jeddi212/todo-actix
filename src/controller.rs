use crate::models::*;
use actix_web::{get, HttpResponse, Responder};

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

#[get("/todos")]
pub async fn get_todos() -> impl Responder {
    HttpResponse::Ok()
        .json(
            TodoList {
                id: 2,
                title: "Todo List 2".to_string()
            }
        )
}