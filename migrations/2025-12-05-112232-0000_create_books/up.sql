-- Your SQL goes here
CREATE TABLE books (
    id SERIAL PRIMARY KEY,
    book_name VARCHAR NOT NULL,
    author VARCHAR NOT NULL
);
