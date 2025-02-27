// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};

use crate::auth::Auth;
use crate::graphql::schema::{ApiSchema, MutationRoot, QueryRoot};

/// GraphQL handler
async fn graphql_handler(
    State(schema): State<ApiSchema>,
    auth: Auth,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner().data(auth)).await.into()
}

/// GraphQL playground handler
async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

/// GraphQL routes
pub fn graphql_routes(schema: ApiSchema) -> Router {
    Router::new()
        .route("/graphql", post(graphql_handler))
        .route("/playground", get(graphql_playground))
        .with_state(schema)
}
