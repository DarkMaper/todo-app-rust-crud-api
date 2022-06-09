-- Your SQL goes here
CREATE TABLE IF NOT EXISTS sessions(
    token VARCHAR(36) NOT NULL,
    user VARCHAR(36) NOT NULL,
    PRIMARY KEY (token),
    FOREIGN KEY (user) REFERENCES users(id) ON DELETE CASCADE
);