#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/")]
fn hello() -> String {
    format!("Hello, world")
}

#[get("/<name>")]
fn hello_name(name: String) -> String {
    format!("Hello, {}", name)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![hello, hello_name])
        .launch();
}
