use rocket::{get};

#[get("/hello")]
pub fn hello() -> &'static str {
    "Hello, world!"
}

#[get("/greet/<name>")]
pub fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}