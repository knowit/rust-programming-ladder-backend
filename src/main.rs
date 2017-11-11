#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[macro_use]
extern crate juniper;

mod api;
mod model;

fn main() {
    rocket::ignite()
        .mount("/", routes![api::index])
        .launch();
}
