-- Add migration script here
CREATE TABLE IF NOT EXISTS profiles (
  profile TEXT NOT NULL PRIMARY KEY,
  name TEXT NOT NULL,
  email TEXT NOT NULL
);
