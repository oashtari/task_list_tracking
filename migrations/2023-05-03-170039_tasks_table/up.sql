-- Your SQL goes here

-- table! {
--     tasks (id) {
--         id -> Int4,
--         user_id -> Int4,
--         task_name -> Varchar,
--         repeat -> Bool,
--         created_at -> Timestamptz,
--         due_at -> Timestamptz,
--         completed_at -> Nullable<Timestamptz>,
--     }
-- }

-- joinable!(tasks -> users (user_id));

CREATE TABLE tasks (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    task_name VARCHAR NOT NULL,
    repeat BOOLEAN NOT NULL,
    created_at TIMESTAMP NOT NULL,
    due_at TIMESTAMP NOT NULL,
    completed_at TIMESTAMP
);
