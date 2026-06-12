-- migrations/<timestamp>_create_todos_table.sql
DROP TABLE IF EXISTS t_todos;

CREATE TABLE t_todos (
    todo_id SERIAL PRIMARY KEY,
    todo_text TEXT NOT NULL,
    user_fk INTEGER NOT NULL,
    FOREIGN KEY (user_fk) REFERENCES t_users(user_id)
);

INSERT INTO t_todos (todo_text, user_fk) VALUES ('Acheter du pain', 1);
INSERT INTO t_todos (todo_text, user_fk) VALUES ('Faire les courses', 1);
INSERT INTO t_todos (todo_text, user_fk) VALUES ('Appeler Jean', 2);