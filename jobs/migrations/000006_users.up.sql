CREATE TABLE users (
    id SERIAL PRIMARY KEY NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    username TEXT UNIQUE NOT NULL,
    hash TEXT NOT NULL
);
