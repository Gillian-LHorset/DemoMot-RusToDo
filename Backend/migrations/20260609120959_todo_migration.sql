-- Add migration script here
DROP TABLE IF EXISTS todos;

CREATE TABLE todos (
    todo_id SERIAL PRIMARY KEY,
    todo_text TEXT
);

INSERT INTO todos (todo_text) VALUES ('Acheter du pain');
INSERT INTO todos (todo_text) VALUES ('Faire les courses');
INSERT INTO todos (todo_text) VALUES ('Appeler Jean');