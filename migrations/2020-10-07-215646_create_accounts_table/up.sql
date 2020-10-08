-- Your SQL goes here
CREATE TABLE accounts (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    currency VARCHAR NOT NULL,
    description VARCHAR NOT NULL
)