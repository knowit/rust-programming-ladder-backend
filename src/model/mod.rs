
pub mod user;

use juniper;

use std::collections::HashMap;

// Our model/in-memory database
pub struct Model {
    users: HashMap<String, user::User>
}

impl Model {
    pub fn new() -> Model {
        Model {
            users: HashMap::new()
        }
    }
}

impl juniper::Context for Model {}

// The root graphql object
pub struct QueryRoot;

graphql_object!(QueryRoot: Model |&self| {
    description: "Root object"
});

pub type Schema = juniper::RootNode<'static, QueryRoot, juniper::EmptyMutation<Model>>;
