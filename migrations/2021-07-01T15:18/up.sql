CREATE TABLE "tasks" (
       id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
       description TEXT NOT NULL,
       complete INTEGER NOT NULL
);

INSERT INTO "tasks" (description, complete) VALUES ("test task", 1);
