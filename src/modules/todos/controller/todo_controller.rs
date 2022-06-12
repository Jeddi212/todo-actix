use actix_web::{get, post, put, delete, web, HttpResponse, Responder, Result};

use super::super::model::todo_model::*;
use super::super::dto::todo_dto::*;
use crate::modules::todos::service::todo_service;

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
            todo_service::find_all()
        )
}

#[get("{id}")]
pub async fn get_todo_by_id(id: web::Path<i32>) -> impl Responder {
    HttpResponse::Ok()
        .json(
            todo_service::find(id.into_inner())
        )
}

#[post("")]
pub async fn post_todo(create_dto: web::Json<PutTodoDTO>) -> Result<impl Responder>{
    Ok(serde_json::to_string(&todo_service::save(create_dto.into_inner()))?)
}

#[put("{id}")]
pub async fn put_todo(id: web::Path<i32>, update_dto: web::Json<PutTodoDTO>) -> Result<String> {
    Ok(serde_json::to_string(&todo_service::update(
        id.into_inner(), 
        update_dto.into_inner()
    ))?)
}

#[delete("{id}")]
pub async fn delete_todo(id: web::Path<i32>) -> Result<String> {
    Ok(format!("Succes delete {} record!", 
        todo_service::remove_one(id.into_inner())
    ))
}
