use std::error::Error;

use async_graphql::{
    http::GraphiQLSource, Context, EmptyMutation, EmptySubscription, Object, Schema,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    response::{self, IntoResponse},
    routing::get,
    Router, Server,
};
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};

async fn graphql_handler(
    schema: Extension<Schema<QueryRoot, EmptyMutation, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn books(&self, ctx: &Context<'_>) -> String {
        return "book".to_string();
    }
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();

    // Connect to the server
    let db = Surreal::new::<Ws>("127.0.0.1:8000")
        .await
        .expect("Failed to connect to server");

    // Signin as a namespace, database, or root user
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .expect("Failed to signin");

    // Select a specific namespace / database
    db.use_ns("test")
        .use_db("test")
        .await
        .expect("Failed to select namespace / database");

    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .layer(Extension(schema));

    println!("GraphiQL IDE: http://localhost:8000");

    Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
