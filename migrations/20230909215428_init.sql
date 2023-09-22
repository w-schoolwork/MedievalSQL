-- Add migration script here

CREATE TABLE users (
	user_id UUID PRIMARY KEY,
	email TEXT UNIQUE NOT NULL,
	totp_secret TEXT UNIQUE NOT NULL
);