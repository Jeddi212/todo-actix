use super::super::model::item_model::*;
use crate::db::db_connection::establish_connection;
use crate::diesel::prelude::*;

pub fn find_all() -> Vec<Items> {
    use crate::schema::items:: dsl::*;

    let connection = establish_connection();
    let results = items
        .limit(50)
        .order(id.asc())
        .load::<Items>(&connection)
        .expect("Error loading items");

    results
}

// pub fn save() -> Vec<Items> {
//     use crate::schema::items;

//     let conn = establish_connection();

//     let new_todo = PutTodoDTO {
//         title: title.to_string()
//     };

//     diesel::insert_into(todos::table)
//         .values(&new_todo)
//         .get_result(&conn)
//         .expect("Error saving new todo")
// }