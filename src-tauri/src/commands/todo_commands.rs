use crate::{models::todo::Todo, services::todo_services};

#[tauri::command]
pub fn create_todo(todo: Todo) {
    todo_services::add_todo(&todo)
}

#[tauri::command]
pub fn get_todos() -> Vec<Todo> {
    todo_services::get_todos().unwrap()
}

#[tauri::command]
pub fn toggle_todo_status(id: i32) {
    todo_services::update_todo(id)
}

#[tauri::command]
pub fn delete_todo(id: i32) {
    todo_services::delete_todo(id)
}
