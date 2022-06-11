use crate::modules::todos::model::todo_model::TodoList;

use super::super::dto::todo_dto::*;
use super::super::model::todo_model::*;
use super::super::repository::todo_repository;

pub fn save(dto: PutTodoDTO) -> TodoList {
    todo_repository::save(PutTodo::from(dto))
}

pub fn update(target_id: i32, dto: PutTodoDTO) -> TodoList {
    todo_repository::update(TodoList {
        id: target_id,
        title: dto.title
    })
}