use crate::modules::todos::model::todo_model::TodoList;

use super::super::dto::todo_dto::*;
use super::super::model::todo_model::*;
use super::super::repository::todo_repository;

pub fn save(dto: PutTodoDTO) -> TodoList {
    todo_repository::save(PutTodo::from(dto))
}