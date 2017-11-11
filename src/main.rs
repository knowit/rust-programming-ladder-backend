#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[macro_use]
extern crate juniper;

extern crate juniper_rocket;

mod api;
mod model;

fn main() {
    rocket::ignite()
        .mount("/", routes![api::graphiql])
        .launch();
}
