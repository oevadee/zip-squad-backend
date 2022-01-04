#[macro_use]
extern crate rocket;

use rocket::form::Form;

#[derive(FromForm)]
struct LoginInput<'r> {
    email: &'r str,
    password: &'r str
}

#[get("/")]
fn hello() -> &'static str {
    "Hello wrld"
}

#[get("/hello/<name>/<age>/<cool>")]
fn hello_with_params(name: &str, age: u8, cool: bool) -> String {
    if cool {
        format!("You're a cool {} year old, {}!", age, name)
    } else {
        format!("{}, we need to talk about your coolness.", name)
    }
}

#[post("/login", data = "<input>")]
fn login(input: Form<LoginInput<'_>>) -> &'static str {
    "Login"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![hello, hello_with_params, login])
}