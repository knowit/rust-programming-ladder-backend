
use juniper::FieldResult;

use model::user::User;

graphql_object!(User: () |&self| {
    description: "A user!",

    field id() -> FieldResult<&String> {
        Ok(&self.id)
    }
});
