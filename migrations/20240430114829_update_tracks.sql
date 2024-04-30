-- Add migration script here
DROP TABLE tracks;

CREATE TABLE IF NOT EXISTS tracks
(
	title VARCHAR NOT NULL,
	artist VARCHAR NOT NULL,
	duration INT NOT NULL,
	path VARCHAR NOT NULL PRIMARY KEY
);