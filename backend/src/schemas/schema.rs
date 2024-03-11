use juniper::{EmptyMutation, EmptySubscription, RootNode};

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    fn api_version() -> &str {
        "1.0"
    }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>, EmptySubscription<()>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new(), EmptySubscription::new())
}
