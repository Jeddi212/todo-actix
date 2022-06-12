use crate::modules::todos::model::todo_model::Todos;

use super::super::dto::todo_dto::*;
use super::super::model::todo_model::*;
use super::super::repository::todo_repository;

pub fn find_all() -> Vec<TodoDTO> {
    todo_repository::find_all()
        .into_iter()
        .map(|t| {
            TodoDTO::from(t)
        })
        .collect()
}

pub fn find(target_id: i32) -> TodoDTO {
    TodoDTO::from(todo_repository::find(target_id))
}

pub fn save(dto: PutTodoDTO) -> TodoDTO {
    TodoDTO::from(todo_repository::save(CreateTodo::from(dto)))
}

pub fn update(target_id: i32, dto: PutTodoDTO) -> TodoDTO {
    TodoDTO::from(
        todo_repository::update(Todos {
            id: target_id,
            title: dto.title
        })
    )
}

pub fn remove_one(target_id: i32) -> usize {
    todo_repository::delete(target_id)
}