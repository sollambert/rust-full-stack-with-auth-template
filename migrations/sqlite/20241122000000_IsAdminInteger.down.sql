CREATE TABLE "users_old" (
    id INTEGER PRIMARY KEY UNIQUE,
    uuid VARCHAR(36) UNIQUE,
    username VARCHAR(24) UNIQUE,
    pass VARCHAR(60),
    email VARCHAR(254) UNIQUE,
    is_admin BOOLEAN
);
INSERT INTO "users_old" (id, uuid, username, pass, email, is_admin)
    SELECT id, uuid, username, pass, email, is_admin FROM "users";
DROP TABLE "users";
ALTER TABLE "users_old" RENAME TO "users";
