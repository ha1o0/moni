use controller::user;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Ok!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index])
    .mount("/user", routes![user::base::hello, user::base::greet])
}