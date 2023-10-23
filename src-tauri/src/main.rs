// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
#[path = "./lib/mod.rs"]
mod lib;
mod models;
mod schema;
mod services;

use commands::todo_commands::create_todo;
use commands::todo_commands::delete_todo;
use commands::todo_commands::get_todos;
use commands::todo_commands::toggle_todo_status;
use lib::db::init;
use schema::create_todo_table;

#[tokio::main]
async fn main() {
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    tauri::Builder::default()
        .setup(|_app| {
            tokio::spawn(async move {
                // initialize the database
                init();

                // create the tables
                create_todo_table();
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            create_todo,
            get_todos,
            toggle_todo_status,
            delete_todo
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
