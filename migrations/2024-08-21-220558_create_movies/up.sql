-- Your SQL goes here
CREATE TABLE movies (
    title VARCHAR(255) PRIMARY KEY,
    genre VARCHAR(255) NOT NULL,
    year INT NOT NULL,
    rating INT
);