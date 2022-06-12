use serde::{Serialize, Deserialize};
use diesel::Queryable;

use crate::schema::todos;
use super::super::dto::todo_dto::*;

#[derive(Serialize)]
pub struct Status {
    pub status: String
}

#[derive(Serialize, Deserialize)]
#[derive(Queryable, Insertable)]
#[derive(AsChangeset, Identifiable)]
#[table_name="todos"]
pub struct Todos {
    pub id: i32,
    pub title: String
}

#[derive(Insertable)]
#[table_name="todos"]
pub struct CreateTodo {
    pub title: String
}

impl From<TodoDTO> for Todos {
    fn from(t: TodoDTO) -> Self {
        Self {
            id: t.id,
            title: t.title
        }
    }
}

impl From<PutTodoDTO> for CreateTodo {
    fn from(p: PutTodoDTO) -> Self {
        Self {
            title: p.title
        }
    }
}
