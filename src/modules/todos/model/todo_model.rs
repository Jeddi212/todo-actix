use serde::{Serialize, Deserialize};
use diesel::Queryable;

use crate::schema::todos;
use super::super::dto::todo_dto::PutTodoDTO;

#[derive(Serialize)]
pub struct Status {
    pub status: String
}

#[derive(Serialize, Deserialize)]
#[derive(Queryable, Insertable)]
#[derive(AsChangeset)]
#[table_name="todos"]
pub struct TodoList {
    pub id: i32,
    pub title: String
}

#[derive(Insertable)]
#[table_name="todos"]
pub struct PutTodo {
    pub title: String
}

impl From<PutTodoDTO> for PutTodo {
    fn from(p: PutTodoDTO) -> Self {
        Self {
            title: p.title
        }
    }
}
