use crate::*;
use crate::models::*;
use self::diesel::prelude::*;

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

pub fn update(target_id: String, new_title: &String) -> TodoList {
    use crate::schema::todos:: dsl::*;

    let target_id: i32 = target_id.trim().parse::<i32>().expect("ID not valid");

    let conn = establish_connection();

    diesel::update(todos.filter(id.eq(target_id)))
        .set(title.eq(new_title))
        .get_result(&conn)
        .expect("Error updating Todos")
}