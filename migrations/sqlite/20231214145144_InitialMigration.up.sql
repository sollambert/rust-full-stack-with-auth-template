-- Add migration script here
CREATE TABLE "users" (
    id INTEGER PRIMARY KEY UNIQUE,
    uuid VARCHAR(36) UNIQUE,
    username VARCHAR(24) UNIQUE,
    pass VARCHAR(60),
    email VARCHAR(254) UNIQUE
);