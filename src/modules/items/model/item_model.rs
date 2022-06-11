use serde::{Serialize, Deserialize};
use diesel::Queryable;

use super::super::dto::item_dto::*;
use crate::schema::*;

#[derive(Serialize, Deserialize)]
#[derive(Queryable)]
pub struct Items {
    pub id: i32,
    pub title: String,
    pub checked: bool,
    pub list_id: i32
}

impl From<ItemDTO> for Items {
    fn from(i: ItemDTO) -> Self {
        Self {
            id: i.id,
            title: i.title,
            checked: i.checked,
            list_id: i.list_id
        }
    }
}