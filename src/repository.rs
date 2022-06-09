use crate::*;
use crate::models::*;
use self::diesel::prelude::*;

pub fn find_all() -> Vec<TodoList> {
    use crate::schema::todos::dsl::*;

    let connection = establish_connection();
    let results = todos
        .limit(5)
        .load::<TodoList>(&connection)
        .expect("Error loading posts");

    let mut vec_todos: Vec<TodoList> = Vec::new();
    for todo in results {
        vec_todos.push(todo);
    }

    vec_todos
}