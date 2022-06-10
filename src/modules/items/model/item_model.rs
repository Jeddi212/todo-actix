use serde::{Serialize, Deserialize};
use diesel::Queryable;
use crate::schema::*;

#[derive(Serialize, Deserialize)]
pub struct TodoItem {
    pub id: i32,
    pub title: String,
    pub checked: bool,
    pub list_id: i32
}