-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE books (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    book_name VARCHAR NOT NULL,
    author VARCHAR NOT NULL
);
