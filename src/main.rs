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
        .manage(model::Model::new())
        .manage(model::Schema::new(
            model::QueryRoot {},
            juniper::EmptyMutation::<model::Model>::new(),
        ))
        .mount("/", routes![
            api::graphiql,
            api::get_graphql_handler,
            api::post_graphql_handler,
        ])
        .launch();
}
