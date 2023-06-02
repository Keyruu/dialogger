use std::env;

use async_graphql::{
    http::GraphiQLSource, Context, EmptyMutation, EmptySubscription, Error, Object, Schema,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    response::{self, IntoResponse},
    routing::get,
    Router, Server,
};
use dotenvy::dotenv;
use model::Person;
use scalar::SurrealID;
use serde::{Deserialize, Serialize};
use simple_logger::SimpleLogger;
use surrealdb::{
    engine::{
        any::connect,
        remote::ws::{Client, Ws},
    },
    opt::auth::Root,
    sql::Thing,
    Surreal,
};
use surrealdb_migrations::MigrationRunner;

use crate::model::Record;

mod model;
mod scalar;

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
    async fn books(&self, ctx: &Context<'_>) -> Result<Vec<Person>, Error> {
        let client = &ctx.data::<SurrealConnection>()?.client;
        let book: Vec<Person> = client.select("book").await?;
        Ok(book)
    }
}

//write function that creates the surreal client and sets a namespace and database
async fn create_client() -> Surreal<surrealdb::engine::any::Any> {
    let url = env::var("SURREAL_URL").expect("SURREAL_URL env is not set");
    let username = env::var("SURREAL_USERNAME").expect("SURREAL_USERNAME env is not set");
    let password = env::var("SURREAL_PASSWORD").expect("SURREAL_PASSWORD env is not set");
    let namespace = env::var("SURREAL_NAMESPACE").expect("SURREAL_NAMESPACE env is not set");
    let database = env::var("SURREAL_DATABASE").expect("SURREAL_DATABASE env is not set");

    let db = connect(url).await.expect("Failed to connect to server");

    // Signin as a namespace, database, or root user
    db.signin(Root {
        username: &username,
        password: &password,
    })
    .await
    .expect("Failed to signin");

    // Select a specific namespace / database
    db.use_ns(namespace)
        .use_db(database)
        .await
        .expect("Failed to select namespace / database");

    // Apply all migrations
    MigrationRunner::new(&db)
        .up()
        .await
        .expect("Failed to apply migrations");

    db
}

struct SurrealConnection {
    client: Surreal<surrealdb::engine::any::Any>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    SimpleLogger::new().env().init().unwrap();

    let db = create_client().await;

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(SurrealConnection { client: db })
        .finish();

    // Connect to the server

    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .layer(Extension(schema));

    let address = env::var("AXUM_LISTEN_ADDRESS").expect("AXUM_LISTEN_ADDRESS env is not set");

    println!("");
    println!("    ____  _       __                           ");
    println!("   / __ \\(_)___ _/ /___  ____ _____ ____  _____");
    println!("  / / / / / __ `/ / __ \\/ __ `/ __ `/ _ \\/ ___/");
    println!(" / /_/ / / /_/ / / /_/ / /_/ / /_/ /  __/ /    ");
    println!("/_____/_/\\__,_/_/\\____/\\__, /\\__, /\\___/_/     ");
    println!("                      /____//____/             ");
    println!("");

    log::info!("🎸 Starting Axum!");
    log::info!("🛝  Playground at http://{}.", address);

    Server::bind(&address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
