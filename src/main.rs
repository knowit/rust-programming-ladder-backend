#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/")]
fn hello() -> String {
    format!("Hello, world")
}

fn main() {
    rocket::ignite().mount("", routes![hello]).launch();
}