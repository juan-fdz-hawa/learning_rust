-- Add migration script here
CREATE TABLE IF NOT EXISTS messages (
    id INTEGER PRIMARY KEY NOT NULL,
    message TEXT NOT NULL
);

INSERT INTO messages (id, message) VALUES (1, 'Hello world');
INSERT INTO messages (id, message) VALUES (2, 'Hello world 2');
INSERT INTO messages (id, message) VALUES (3, 'Hello world 3');
INSERT INTO messages (id, message) VALUES (4, 'Hello world 4');