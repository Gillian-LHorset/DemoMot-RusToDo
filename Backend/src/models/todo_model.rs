use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct TodoItem {
    pub todo_id: i32,
    pub todo_text: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateTodo {
    pub todo_text: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateTodo {
    pub todo_text: String,
}

#[derive(Serialize)]
pub struct Message {
    pub message: String,
}