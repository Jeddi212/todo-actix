use serde::{Serialize, Deserialize};

use super::super::model::todo_model::*;

#[derive(Serialize, Deserialize)]
pub struct TodoDTO {
    pub id: i32,
    pub title: String
}

#[derive(Serialize, Deserialize)]
pub struct PutTodoDTO {
    pub title: String
}

impl From<Todos> for TodoDTO {
    fn from(t: Todos) -> Self {
        Self {
            id: t.id,
            title: t.title
        }
    }
}

impl From<CreateTodo> for PutTodoDTO {
    fn from(t: CreateTodo) -> Self {
        Self {
            title: t.title
        }
    }
}