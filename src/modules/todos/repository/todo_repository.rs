use super::super::model::todo_model::*;
use crate::db::db_connection::establish_connection;
use crate::diesel::prelude::*;

pub fn find_all() -> Vec<TodoList> {
    use crate::schema::todos:: dsl::*;

    let connection = establish_connection();
    let results = todos
        .limit(50)
        .order(id.asc())
        .load::<TodoList>(&connection)
        .expect("Error loading posts");

    let mut vec_todos: Vec<TodoList> = Vec::new();
    for todo in results {
        vec_todos.push(todo);
    }

    vec_todos
}

pub fn find(target_id: i32) -> TodoList {
    use crate::schema::todos:: dsl::*;  

    let connection = establish_connection();
    todos.find(target_id).get_result(&connection).expect("Todos not found")
}

pub fn save(title: &String) -> TodoList {
    use crate::schema::todos;

    let conn = establish_connection();

    let new_todo = PutTodoDTO {
        title: title.to_string()
    };

    diesel::insert_into(todos::table)
        .values(&new_todo)
        .get_result(&conn)
        .expect("Error saving new todo")
}

pub fn update(target_id: i32, new_title: &String) -> TodoList {
    use crate::schema::todos:: dsl::*;

    let conn = establish_connection();

    diesel::update(todos.filter(id.eq(target_id)))
        .set(title.eq(new_title))
        .get_result(&conn)
        .expect("Error updating Todos")
}

pub fn delete(target_id: i32) -> usize {
    use crate::schema::todos:: dsl::*;

    let conn = establish_connection();

    diesel::delete(todos.filter(id.eq(target_id))).execute(&conn).expect("Error deleting")
}