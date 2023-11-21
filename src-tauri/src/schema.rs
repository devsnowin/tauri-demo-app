use crate::lib::db;

pub fn create_todo_table() {
    let db = db::establish_db_connection();
    let query = "CREATE TABLE IF NOT EXISTS todo (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        text TEXT NOT NULL,
        is_completed BOOLEAN DEFAULT 0,
        created_at TIMESTAMPTZ DEFAULT (datetime('now'))
    );";
    db.execute(query, ())
        .unwrap_or_else(|_| panic!("Error creating todo table"));
}
