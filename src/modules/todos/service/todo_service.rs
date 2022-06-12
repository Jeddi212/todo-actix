use crate::modules::todos::model::todo_model::TodoList;

use super::super::dto::todo_dto::*;
use super::super::model::todo_model::*;
use super::super::repository::todo_repository;

pub fn find_all() -> Vec<TodoList> {
    todo_repository::find_all()
}

pub fn find(target_id: i32) -> TodoList {
    todo_repository::find(target_id)
}

pub fn save(dto: PutTodoDTO) -> TodoList {
    todo_repository::save(PutTodo::from(dto))
}

pub fn update(target_id: i32, dto: PutTodoDTO) -> TodoList {
    todo_repository::update(TodoList {
        id: target_id,
        title: dto.title
    })
}

pub fn remove_one(target_id: i32) -> usize {
    todo_repository::delete(target_id)
}