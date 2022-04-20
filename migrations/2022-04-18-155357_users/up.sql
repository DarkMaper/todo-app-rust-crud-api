-- Your SQL goes here
CREATE TABLE IF NOT EXISTS users(
    email VARCHAR(30) NOT NULL PRIMARY KEY,
    password VARCHAR(60) NOT NULL
);