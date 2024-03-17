-- Your SQL goes here
CREATE TABLE passwords (
	id SERIAL PRIMARY KEY,
	name VARCHAR NOT NULL,
	value VARCHAR NOT NULL
);
