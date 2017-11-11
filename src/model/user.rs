use juniper;

use model;

pub struct User {
    pub id: String,
}

graphql_object!(User: model::Model |&self| {
    description: "A user!",

    field id() -> juniper::FieldResult<&String> {
        Ok(&self.id)
    }
});
