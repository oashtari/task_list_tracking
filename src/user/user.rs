use chrono::NaiveDate;
// use postal::{PostalCode, Country};
use diesel::prelude::*;
use diesel::Queryable;

// #[derive(Queryable)] will generate all of the code needed to load a Post struct from a SQL query.
// Using #[derive(Queryable)] assumes that the order of fields on the Post struct 
// matches the columns in the posts table, so make sure to define them in the order seen in the schema.rs file.

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub birthdate: NaiveDate,
    // pub zip_code: PostalCode<Country>,
}

impl User {
    pub fn new(id: i32, first_name: String, last_name: String, email: String, birthdate: NaiveDate, 
        // postal_code: PostalCode
    ) -> Self {
        User {
            id, 
            first_name,
            last_name,
            email,
            birthdate,
            // postal_code,s
        }
    }

    // let user = User::new(
    //     "John".to_string(),
    //     "Doe".to_string(),
    //     "john.doe@example.com".to_string(),
    //     NaiveDate::from_ymd(1980, 1, 1),
    //     PostalCode::new("12345".to_string()).unwrap(),
    // );
    
}
