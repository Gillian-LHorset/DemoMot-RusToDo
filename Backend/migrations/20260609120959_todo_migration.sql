-- migrations/<timestamp>_create_todos_table.sql
DROP TABLE IF EXISTS todos;

CREATE TABLE users (
    user_id SERIAL PRIMARY KEY,
    username VARCHAR(51)
);

CREATE TABLE todos (
    todo_id SERIAL PRIMARY KEY,
    todo_text TEXT NOT NULL,
    user_fk INTEGER NOT NULL,
    FOREIGN KEY (user_fk) REFERENCES users(user_id)
);

INSERT INTO users (username) VALUES ('Kyell');
INSERT INTO users (username) VALUES ('Albert');

INSERT INTO todos (todo_text, user_fk) VALUES ('Acheter du pain', 1);
INSERT INTO todos (todo_text, user_fk) VALUES ('Faire les courses', 1);
INSERT INTO todos (todo_text, user_fk) VALUES ('Appeler Jean', 2);