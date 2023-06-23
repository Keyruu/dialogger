use std::env;

use crate::parse::parse_srt;
use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    response::{self, IntoResponse},
    routing::{get, post},
    Router, Server,
};
use dotenvy::dotenv;
use model::{MutationRoot, QueryRoot};
use simple_logger::SimpleLogger;
use sqlx::{migrate, postgres::PgPoolOptions};

pub mod model;
pub mod parse;

async fn graphql_handler(
    schema: Extension<Schema<QueryRoot, MutationRoot, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    SimpleLogger::new().env().init().unwrap();

    parse_srt("resources/wixxer.srt");

    // let db = create_client().await;
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL env is not set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to postgres");

    migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let schema = Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(pool)
    .finish();

    // Connect to the server

    let app = Router::new()
        .route("/", get(graphiql))
        .route("/graphql", post(graphql_handler))
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
