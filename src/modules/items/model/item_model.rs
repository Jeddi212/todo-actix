use serde::{Serialize, Deserialize};
use diesel::Queryable;

use crate::schema::items;
use super::super::dto::item_dto::*;

#[derive(Serialize, Deserialize)]
#[derive(Queryable, Insertable)]
#[derive(AsChangeset, Identifiable)]
#[table_name="items"]
pub struct Items {
    pub id: i32,
    pub title: String,
    pub checked: bool,
    pub list_id: i32
}

#[derive(Queryable, Insertable)]
#[table_name="items"]
pub struct CreateItems {
    pub title: String,
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

impl From<CreateItemDTO> for CreateItems {
    fn from(i: CreateItemDTO) -> Self {
        Self {
            title: i.title,
            list_id: i.list_id
        }
    }
}