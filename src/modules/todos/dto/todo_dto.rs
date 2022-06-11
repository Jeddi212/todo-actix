use serde::{Serialize, Deserialize};

use super::super::model::todo_model::*;

#[derive(Serialize, Deserialize)]
pub struct PutTodoDTO {
    pub title: String
}

impl From<PutTodo> for PutTodoDTO {
    fn from(p: PutTodo) -> Self {
        Self {
            title: p.title
        }
    }
}