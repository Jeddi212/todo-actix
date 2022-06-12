use serde::{Serialize, Deserialize};

use super::super::model::item_model::*;

#[derive(Serialize, Deserialize)]
pub struct ItemDTO {
    pub id: i32,
    pub title: String,
    pub checked: bool,
    pub list_id: i32
}

#[derive(Serialize, Deserialize)]
pub struct CreateItemDTO {
    pub title: String,
    pub list_id: i32
}

impl From<Items> for ItemDTO {
    fn from(i: Items) -> Self {
        Self {
            id: i.id,
            title: i.title,
            checked: i.checked,
            list_id: i.list_id
        }
    }
}