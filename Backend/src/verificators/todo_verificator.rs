use crate::models::todo_model::{CreateTodo, UpdateTodo};

pub fn verify_create(todo: &CreateTodo) -> bool {
    !todo.todo_text.is_empty() && todo.todo_text.len() <= 255
}

pub fn verify_update(todo: &UpdateTodo) -> bool {
    !todo.todo_text.is_empty() && todo.todo_text.len() <= 255
}