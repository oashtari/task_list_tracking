// #![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Welcome to your task tracking home!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}


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


// A Note on Using Migrations in Production:
// When preparing your app for use in production, you may want to run your migrations during the application’s initialization phase. 
// You may also want to include the migration scripts as a part of your code, to avoid having to copy them to your deployment location/image etc.

// The diesel_migrations crate provides the embed_migrations! macro, allowing you to embed migration scripts in the final binary. 
// Once your code uses it, you can simply include connection.run_pending_migrations(MIGRATIONS) at the start of your main function to run migrations every time the application starts.
