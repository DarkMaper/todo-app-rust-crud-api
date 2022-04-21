-- Your SQL goes here
CREATE TABLE IF NOT EXISTS users(
    id VARCHAR(36) NOT NULL,
    email VARCHAR(30) NOT NULL UNIQUE,
    password VARCHAR(60) NOT NULL,
    PRIMARY KEY (id)
);