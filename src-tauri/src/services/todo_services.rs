use rusqlite::params;

use crate::lib::db::establish_db_connection;
use crate::models::todo::Todo;

pub fn add_todo(todo: &Todo) {
    let db = &mut establish_db_connection();

    let query = "INSERT INTO todo (text) VALUES (?1);";

    db.execute(query, (&todo.text,))
        .unwrap_or_else(|_| panic!("Error in inserting todo"));
}

pub fn get_todos() -> Result<Vec<Todo>, rusqlite::Error> {
    let db = &mut establish_db_connection();
    let query = "SELECT * FROM todo;";
    let mut stmt = db.prepare(query).unwrap();
    let todo_iter = stmt
        .query_map([], |row| {
            Ok(Todo {
                id: row.get(0)?,
                text: row.get(1)?,
                is_completed: row.get(2)?,
                created_at: row.get(3)?,
            })
        })
        .unwrap_or_else(|_| panic!("Error selecting todos from the database"));

    let mut todos = Vec::new();
    for todo in todo_iter {
        todos.push(todo.unwrap())
    }

    Ok(todos)
}

pub fn update_todo(id: i32) {
    let db = &mut establish_db_connection();

    // First, query the current value of is_completed
    let select_query = "SELECT is_completed FROM todo WHERE id = ?1;";
    let current_value: Result<bool, rusqlite::Error> =
        db.query_row(select_query, params![id], |row| row.get(0));

    match current_value {
        Ok(current_status) => {
            // Calculate the new value by inverting the current value
            let new_status = !current_status;

            // Update the todo with the new value
            let update_query = "UPDATE todo SET is_completed = ?1 WHERE id = ?2;";
            db.execute(update_query, params![new_status, id]).unwrap();
        }
        Err(e) => {
            // Handle the error, e.g., todo not found
            panic!("Error: {:?}", e)
        }
    }
}

pub fn delete_todo(id: i32) {
    let db = &mut establish_db_connection();
    let query = "DELETE FROM todo WHERE id = ?1;";

    match db.execute(query, (&id,)) {
        Ok(rows_affected) => {
            if rows_affected == 0 {
                println!("No todo with id {} found.", id);
            } else {
                println!("Todo with id {} deleted successfully.", id);
            }
        }
        Err(err) => {
            eprintln!("Error in deleting todo: {:?}", err);
        }
    }
}
