use juniper::{graphql_object, EmptyMutation, EmptySubscription, RootNode};
use juniper_cfworker::GraphQLHandler;
use worker::*;

struct Context;

impl juniper::Context for Context {}

struct Query;

#[graphql_object]
impl Query {
    fn hello(world: String) -> String {
        format!("Hello, {}!", world)
    }
}

// A root schema consists of a query and a mutation.
// Request queries can be executed against a RootNode.
type Schema = RootNode<'static, Query, EmptyMutation, EmptySubscription>;

#[event(fetch)]
pub async fn main(req: Request, env: Env) -> Result<Response> {
    let root = Schema::new(Query, EmptyMutation::new(), EmptySubscription::new());
    let handler = GraphQLHandler::new(root, ());
    handler.fetch(req, env).await
}
