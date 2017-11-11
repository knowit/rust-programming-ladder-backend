
pub mod user;

use juniper;

use std::collections::HashMap;

pub struct QueryRoot;

// Our model/database
pub struct Model {
    users: HashMap<String, user::User>
}

graphql_object!(QueryRoot: Model |&self| {
});

impl juniper::Context for Model {}

pub type Schema = juniper::RootNode<'static, QueryRoot, juniper::EmptyMutation<Model>>;
