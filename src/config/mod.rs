use actix_web::{web};

use crate::modules::todos::controller::todo_controller::*;

pub fn todo_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/todo")
                    .service(get_todo)
                    .service(get_todo_by_id)
                    .service(post_todo)
                    .service(put_todo)
                    .service(delete_todo)
    );
}