-- Add migration script here
CREATE TABLE IF NOT EXISTS "groups" (
    name TEXT PRIMARY KEY CHECK (name SIMILAR TO '[a-z0-9]+')
);
