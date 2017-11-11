
pub mod user;

use juniper;

pub type Model = user::User;
pub type Schema = juniper::RootNode<'static, Model, juniper::EmptyMutation<Model>>;
