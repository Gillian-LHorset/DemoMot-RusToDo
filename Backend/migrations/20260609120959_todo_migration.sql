-- Add migration script here
CREATE TABLE todos (
    todo_id SERIAL PRIMARY KEY,
    todo_text TEXT
)