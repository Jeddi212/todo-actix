use serde::{Serialize, Deserialize};
use tokio_pg_mapper_derive::PostgresMapper;
use diesel::Queryable;
use crate::schema::*;

#[derive(Serialize)]
pub struct Status {
    pub status: String
}

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table="todo_list")]
#[derive(Queryable)]
pub struct TodoList {
    pub id: i32,
    pub title: String
}

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table="todo_list")]
#[derive(Insertable)]
#[table_name="todos"]
pub struct CreateTodo {
    pub title: String
}

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table="todo_item")]
pub struct TodoItem {
    pub id: i32,
    pub title: String,
    pub checked: bool,
    pub list_id: i32
}
