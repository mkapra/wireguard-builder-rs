use async_graphql::*;

pub type GrahpQLSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn create_schema() -> GrahpQLSchema {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish()
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Says hello
    async fn say_hello(&self) -> String {
        "Hello World!".to_owned()
    }
}
