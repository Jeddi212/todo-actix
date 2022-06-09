use actix_web::{get, post, put, delete, web, HttpResponse, Responder, Result};
use serde::{self, Serialize, Deserialize};

use crate::models::*;
use crate::repository::*;

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
            find_all()
        )
}

#[get("/todo/{id}")]
pub async fn get_todo_by_id(id: web::Path<i32>) -> impl Responder {
    HttpResponse::Ok()
        .json(
            find(id.into_inner())
        )
}

#[post("/todo")]
pub async fn post_todo(create_dto: web::Json<PutTodoDTO>) -> Result<impl Responder>{
    Ok(serde_json::to_string(&save(&create_dto.title))?)
}

#[put("/todo/{id}")]
pub async fn put_todo(id: web::Path<i32>, update_dto: web::Json<PutTodoDTO>) -> Result<String> {
    Ok(serde_json::to_string(&update(id.into_inner(), &update_dto.title))?)
}

#[delete("/todo/{id}")]
pub async fn delete_todo(id: web::Path<i32>) -> Result<String> {
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