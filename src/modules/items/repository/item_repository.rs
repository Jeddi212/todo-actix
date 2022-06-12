use super::super::model::item_model::*;
use crate::db::db_connection::establish_connection;
use crate::diesel::prelude::*;

pub fn find_all() -> Vec<Items> {
    use crate::schema::items:: dsl::*;

    let connection = establish_connection();
    items.limit(50)
        .order(id.asc())
        .load::<Items>(&connection)
        .expect("Error loading items")
}

pub fn find(target_id: i32) -> Items {
    use crate::schema::items:: dsl::*;  

    let connection = establish_connection();
    items.find(target_id).get_result(&connection).expect("Items not found")
}

pub fn save(items: Vec<CreateItems>) -> Vec<Items> {
    use crate::schema::items;

    let conn = establish_connection();

    let x = conn.build_transaction()
        .read_write()
        .run::<_, diesel::result::Error, _>(|| {
            let result = diesel::insert_into(items::table)
                .values(&items)
                .load(&conn)
                .expect("Error saving new items");
            Ok(result)
        });
    
    x.expect("Failed to save new items")

}

pub fn update(item: Items) -> Items {
    // use crate::schema::items:: dsl::*;
    let conn = establish_connection();

    item.save_changes::<Items>(&conn).expect("Error updating Item")
    // diesel::update(items.find(item.id))
    //     .set(&item)
    //     .get_result(&conn)
    //     .expect("Error updating Items")
}

pub fn delete(target_id: i32) -> usize {
    use crate::schema::items:: dsl::*;

    let conn = establish_connection();

    diesel::delete(items.filter(id.eq(target_id))).execute(&conn).expect("Error delete item")
}