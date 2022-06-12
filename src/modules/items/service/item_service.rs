use super::super::dto::item_dto::*;
use super::super::model::item_model::*;
use super::super::repository::item_repository;

pub fn find_all() -> Vec<ItemDTO> {

    item_repository::find_all()
        .into_iter()
        .map(|items| ItemDTO::from(items))
        .collect()

}

pub fn find(target_id: i32) -> ItemDTO {
    ItemDTO::from(item_repository::find(target_id))
}

pub fn save(create_dto: Vec<CreateItemDTO>) -> Vec<ItemDTO> {

    item_repository::save(
        create_dto.into_iter().map(|i| {
            CreateItems::from(i)
        })
        .collect()
    )
        .into_iter()
        .map(|items| {
            ItemDTO::from(items)
        })
        .collect()

}

pub fn update(target_id: i32, update_dto: PutItemDTO) -> Items {

    let mut item: Items = item_repository::find(target_id);

    item.apply_update(update_dto);
    
    item_repository::update(item)

}

pub fn remove_one(target_id: i32) -> usize {

    item_repository::delete(target_id)

}