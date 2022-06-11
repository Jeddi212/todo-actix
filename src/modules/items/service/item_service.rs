use super::super::dto::item_dto::*;
use super::super::model::item_model::*;
use super::super::repository::item_repository;

pub fn find_all() -> Vec<ItemDTO> {

    let vec_items: Vec<Items> = item_repository::find_all();
    vec_items
        .into_iter()
        .map(|items| ItemDTO::from(items))
        .collect()

}