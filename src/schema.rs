table! {
    users (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        birthdate -> Date,
        postal_code -> Postal,
    }
}

table! {
    tasks (id) {
        id -> Int4,
        user_id -> Int4,
        task_name -> Varchar,
        repeat -> Bool,
        created_at -> Timestamp,
        due_at -> Nullable<Timestamp>,
        completed_at -> Nullable<Timestamp>,
    }
}

joinable!(tasks -> users (user_id));

allow_tables_to_appear_in_same_query!(users, tasks);
