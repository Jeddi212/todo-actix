use actix_web::{get, post, put, delete, web, HttpResponse, Responder, Result};

use super::super::service::item_service;
use super::super::dto::item_dto::*;

#[get("")]
pub async fn get_item() -> impl Responder {
    HttpResponse::Ok()
        .json(
            item_service::find_all()
        )
}

#[get("{id}")]
pub async fn get_item_by_id(id: web::Path<i32>) -> impl Responder {
    HttpResponse::Ok()
        .json(
            item_service::find(id.into_inner())
        )
}

#[post("")]
pub async fn post_item(create_dto: web::Json<Vec<CreateItemDTO>>) -> Result<impl Responder>{
    Ok(serde_json::to_string(&item_service::save(create_dto.into_inner()))?)
}

// #[put("{id}")]
// pub async fn put_item(id: web::Path<i32>, update_dto: web::Json<PutItemDTO>) -> Result<String> {
//     Ok(serde_json::to_string(&update(id.into_inner(), &update_dto.title))?)
// }

// #[delete("{id}")]
// pub async fn delete_item(id: web::Path<i32>) -> Result<String> {
//     Ok(format!("Succes delete {} record!", 
//         delete(id.into_inner())
//     ))
// }
