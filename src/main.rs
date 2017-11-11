#![feature(plugin)]
#![plugin(rocket_codegen)]

mod api;

extern crate rocket;

fn main() {
    rocket::ignite()
        .mount("/", routes![api::index])
        .launch();
}
