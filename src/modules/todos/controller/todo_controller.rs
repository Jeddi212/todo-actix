use actix_web::{get, post, put, delete, web, HttpResponse, Responder, Result};

use super::super::model::todo_model::*;
use crate::modules::todos::repository::todo_repository::*;

#[get("/")]
pub async fn status() -> impl Responder {
    HttpResponse::Ok()
        .json(
            Status { 
                status: "UP".to_string()
            }
        )
}

#[get("")]
pub async fn get_todo() -> impl Responder {
    HttpResponse::Ok()
        .json(
            find_all()
        )
}

#[get("{id}")]
pub async fn get_todo_by_id(id: web::Path<i32>) -> impl Responder {
    HttpResponse::Ok()
        .json(
            find(id.into_inner())
        )
}

#[post("")]
pub async fn post_todo(create_dto: web::Json<PutTodoDTO>) -> Result<impl Responder>{
    Ok(serde_json::to_string(&save(&create_dto.title))?)
}

#[put("{id}")]
pub async fn put_todo(id: web::Path<i32>, update_dto: web::Json<PutTodoDTO>) -> Result<String> {
    Ok(serde_json::to_string(&update(id.into_inner(), &update_dto.title))?)
}

#[delete("{id}")]
pub async fn delete_todo(id: web::Path<i32>) -> Result<String> {
    Ok(format!("Succes delete {} record!", 
        delete(id.into_inner())
    ))
}
