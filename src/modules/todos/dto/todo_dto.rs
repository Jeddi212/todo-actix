use serde::{Serialize, Deserialize};

use super::super::model::todo_model::*;

#[derive(Serialize, Deserialize)]
pub struct PutTodoDTO {
    pub title: String
}

impl From<CreateTodo> for PutTodoDTO {
    fn from(p: CreateTodo) -> Self {
        Self {
            title: p.title
        }
    }
}