-- Your SQL goes here
DROP TABLE accounts;

CREATE TABLE accounts (
    id VARCHAR PRIMARY KEY,
    name VARCHAR NOT NULL,
    currency VARCHAR NOT NULL,
    description VARCHAR NOT NULL
)