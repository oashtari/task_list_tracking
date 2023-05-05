-- Your SQL goes here

table! {
    tasks (id) {
        id -> Int4,
        user_id -> Int4,
        task_name -> Varchar,
        repeat -> Bool,
        created_at -> Timestamptz,
        due_at -> Timestamptz,
        completed_at -> Nullable<Timestamptz>,
    }
}

joinable!(tasks -> users (user_id));
