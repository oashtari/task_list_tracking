-- Your SQL goes here

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
