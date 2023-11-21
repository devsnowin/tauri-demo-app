use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: Option<i32>,
    pub text: String,
    pub is_completed: Option<bool>,
    pub created_at: Option<String>,
}
