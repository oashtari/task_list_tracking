use chrono::NaiveDate;
use postal::{PostalCode, Country};

pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub birthdate: NaiveDate,
    pub zip_code: PostalCode<Country>,
}

impl User {
    pub fn new(first_name: String, last_name: String, email: String, birthdate: NaiveDate, postal_code: PostalCode) -> Self {
        User {
            first_name,
            last_name,
            email,
            birthdate,
            postal_code,
        }
    }
}
