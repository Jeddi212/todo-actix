use serde::{Serialize, Deserialize};
use diesel::Queryable;
use crate::schema::todos;

#[derive(Serialize)]
pub struct Status {
    pub status: String
}

#[derive(Serialize, Deserialize)]
#[derive(Queryable)]
pub struct TodoList {
    pub id: i32,
    pub title: String
}

#[derive(Serialize, Deserialize)]
#[derive(Insertable)]
#[table_name="todos"]
pub struct PutTodoDTO {
    pub title: String
}
