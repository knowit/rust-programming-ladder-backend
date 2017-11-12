
pub mod user;

use juniper;

use std::collections::HashMap;

// Our model/in-memory database
pub struct Model {
    users: HashMap<String, user::User>
}

impl Model {
    pub fn new() -> Model {
        let mut model = Model {
            users: HashMap::new()
        };

        model.add_user(user::User::new("1000".to_string()));

        model
    }

    fn add_user(&mut self, user: user::User) {
        self.users.insert(user.id.clone(), user);
    }
}

impl juniper::Context for Model {}

// The root graphql object
pub struct QueryRoot;

graphql_object!(QueryRoot: Model |&self| {
    description: "Root object",

    field user(&executor, id: String as "id of the user") -> Option<&user::User> {
        executor.context().users.get(&id)
    }
});

pub type Schema = juniper::RootNode<'static, QueryRoot, juniper::EmptyMutation<Model>>;
