use std::collections::HashMap;

use juniper::{
    http::GraphQLRequest, GraphQLType, GraphQLTypeAsync, InputValue, RootNode, ScalarValue,
};
use serde::Deserialize;
use serde_json::Value;
use worker::{Env, Method, Request, Response, Result};

#[derive(Deserialize)]
struct GraphQLPOSTRequestBody {
    query: String,
    operation_name: Option<String>,
    variables: Value,
}

pub struct GraphQLHandler<S, QueryT, MutationT, SubscriptionT>
where
    QueryT: GraphQLTypeAsync<S>,
    QueryT::TypeInfo: Sync,
    QueryT::Context: Sync,
    MutationT: GraphQLTypeAsync<S, Context = QueryT::Context>,
    MutationT::TypeInfo: Sync,
    SubscriptionT: GraphQLType<S, Context = QueryT::Context> + Sync,
    SubscriptionT::TypeInfo: Sync,
    S: ScalarValue + Send + Sync,
{
    root: RootNode<'static, QueryT, MutationT, SubscriptionT, S>,
    context: QueryT::Context,
}

impl<S, QueryT, MutationT, SubscriptionT> GraphQLHandler<S, QueryT, MutationT, SubscriptionT>
where
    QueryT: GraphQLTypeAsync<S>,
    QueryT::TypeInfo: Sync,
    QueryT::Context: Sync,
    MutationT: GraphQLTypeAsync<S, Context = QueryT::Context>,
    MutationT::TypeInfo: Sync,
    SubscriptionT: GraphQLType<S, Context = QueryT::Context> + Sync,
    SubscriptionT::TypeInfo: Sync,
    S: ScalarValue + Send + Sync,
{
    pub fn new(
        root: RootNode<'static, QueryT, MutationT, SubscriptionT, S>,
        context: QueryT::Context,
    ) -> Self {
        Self { root, context }
    }

    pub async fn fetch(&self, mut req: Request, _: Env) -> Result<Response> {
        let graphql_request = match req.method() {
            Method::Get => {
                let url = req.url()?;
                let query_pairs = url.query_pairs();
                let query_params: HashMap<_, _> = query_pairs.into_owned().collect();
                let query = query_params.get("query").unwrap().to_owned(); // TODO: make this a valid error
                let variables: Option<InputValue<S>> = query_params
                    .get("variables")
                    .map(|x| &x[..])
                    .map(serde_json::from_str)
                    .transpose()?;
                let operation_name = query_params.get("operation_name").map(ToOwned::to_owned);
                GraphQLRequest::new(query, operation_name, variables)
            }
            Method::Post => {
                let body = req.text().await?;
                let body: GraphQLPOSTRequestBody = serde_json::from_str(&body)?;
                let query = body.query;
                let operation_name = body.operation_name;
                let variables: Option<InputValue<S>> = serde_json::from_value(body.variables)?;
                GraphQLRequest::new(query, operation_name, variables)
            }
            _ => {
                return Response::error("Method not allowed", 405);
            }
        };
        let graphql_response = graphql_request.execute(&self.root, &self.context).await;
        let status = if graphql_response.is_ok() { 200 } else { 400 };
        let res = Response::from_json(&graphql_response)?.with_status(status);
        Ok(res)
    }
}
