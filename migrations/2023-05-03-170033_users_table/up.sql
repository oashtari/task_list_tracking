-- Your SQL goes here

-- table! {
--     users (id) {
--         id -> Int4,
--         first_name -> Varchar,
--         last_name -> Varchar,
--         email -> Varchar,
--         birthdate -> Date,
--         postal_code -> Postal,
--     }
-- }


CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    birthdate DATE NOT NULL,
    postal_code VARCHAR NOT NULL
);
