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

    // let user = User::new(
    //     "John".to_string(),
    //     "Doe".to_string(),
    //     "john.doe@example.com".to_string(),
    //     NaiveDate::from_ymd(1980, 1, 1),
    //     PostalCode::new("12345".to_string()).unwrap(),
    // );
    
}
