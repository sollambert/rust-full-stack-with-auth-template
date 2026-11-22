-- The sqlx Any driver cannot decode SQLite's BOOLEAN declared type, so store is_admin
-- as INTEGER (0/1). SQLite cannot alter a column type in place: rebuild the table.
CREATE TABLE "users_new" (
    id INTEGER PRIMARY KEY UNIQUE,
    uuid VARCHAR(36) UNIQUE,
    username VARCHAR(24) UNIQUE,
    pass VARCHAR(60),
    email VARCHAR(254) UNIQUE,
    is_admin INTEGER NOT NULL DEFAULT 0
);
INSERT INTO "users_new" (id, uuid, username, pass, email, is_admin)
    SELECT id, uuid, username, pass, email, COALESCE(is_admin, 0) FROM "users";
DROP TABLE "users";
ALTER TABLE "users_new" RENAME TO "users";
