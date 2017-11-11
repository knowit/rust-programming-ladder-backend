use juniper;

pub struct User {
    pub id: String,
}

graphql_object!(User: () |&self| {
    description: "A user!",

    field id() -> juniper::FieldResult<&String> {
        Ok(&self.id)
    }
});
