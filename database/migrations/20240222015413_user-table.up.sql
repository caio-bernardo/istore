-- Add up migration script here
CREATE TABLE user (
  id integer primary key,
  username text unique,
  password text
);
