
use rocket;
use juniper_rocket;

use model;

#[get("/")]
fn graphiql() -> rocket::response::content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[get("/graphql?<request>")]
fn get_graphql_handler(
    context: rocket::State<model::Model>,
    request: juniper_rocket::GraphQLRequest,
    schema: rocket::State<model::Schema>
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

#[post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: rocket::State<model::Model>,
    request: juniper_rocket::GraphQLRequest,
    schema: rocket::State<model::Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}
