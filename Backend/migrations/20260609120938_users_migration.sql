DROP TABLE IF EXISTS t_users;

CREATE TABLE t_users (
   user_id SERIAL PRIMARY KEY,
   username VARCHAR(51),
   password_hash TEXT NOT NULL
);

INSERT INTO t_users (username, password_hash) VALUES ('Kyell', 'jspmdr');
INSERT INTO t_users (username, password_hash) VALUES ('Albert', 'jspmdr');