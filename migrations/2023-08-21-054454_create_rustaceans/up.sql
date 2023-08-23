-- Your SQL goes here -- ID will be Nullable in Schema, remove Nullable
CREATE TABLE rustaceans (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)
