-- Add migration script here
CREATE TABLE IF NOT EXISTS events (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    description TEXT NOT NULL,
    "group" TEXT NOT NULL,
    start TIMESTAMPTZ NOT NULL,
    "end" TIMESTAMPTZ NOT NULL CHECK ("end" > start),
    FOREIGN KEY ("group") REFERENCES "groups"(name)
);
