-- Your SQL goes here

CREATE TABLE users (
  id VARCHAR PRIMARY KEY,
  first_name VARCHAR NOT NULL,
  last_name VARCHAR NOT NULL,
  mail_address VARCHAR NOT NULL,
  age SMALLINT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL,
  updated_at TIMESTAMPTZ NOT NULL
)
