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
