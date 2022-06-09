-- Your SQL goes here
CREATE TABLE IF NOT EXISTS notes(
    id VARCHAR(36) NOT NULL,
    title VARCHAR(20) NOT NULL,
    content TEXT NOT NULL,
    user VARCHAR(36) NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (user) REFERENCES users(id) ON DELETE CASCADE
);