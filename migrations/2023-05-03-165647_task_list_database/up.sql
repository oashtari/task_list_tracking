#[macro_use]
extern crate diesel;

embed_migrations!();

fn main() {
    let connection = establish_connection();

    embedded_migrations::run(&connection).expect("Failed to run database migrations");
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
