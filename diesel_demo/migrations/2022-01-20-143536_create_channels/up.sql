-- Your SQL goes here
CREATE TABLE channels (
  id VARCHAR NOT NULL PRIMARY KEY,
  title VARCHAR NOT NULL,
  'description' TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 'f'
)