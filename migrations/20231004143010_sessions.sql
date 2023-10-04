-- Add migration script here

CREATE TABLE session (
  user_id UUID PRIMARY KEY,
  FOREIGN KEY (user_id) REFERENCES users(user_id),
  secret TEXT NOT NULL
);