use juniper;

use model;

pub struct User {
    pub id: String,
}

impl User {
    pub fn new(id: String) -> User {
        User {
            id: id
        }
    }
}

// model::Model here is the Context object - the functions should be able to call functions on it.
graphql_object!(User: model::Model |&self| {
    description: "A user!",

    field id() -> juniper::FieldResult<&String> {
        Ok(&self.id)
    }
});
