use async_graphql::*;
use serde::{Deserialize, Serialize};

use crate::{scalar::SurrealID, SurrealConnection};

#[derive(Debug, Serialize, Deserialize, SimpleObject)]
pub struct Movie {
    pub id: Option<SurrealID>,
    pub name: String,
}

#[derive(Default)]
pub struct MovieQuery;

#[Object]
impl MovieQuery {
    async fn movies(&self, ctx: &Context<'_>) -> Result<Vec<Movie>, Error> {
        let client = &ctx.data::<SurrealConnection>()?.client;
        let movies: Vec<Movie> = client.select("movie").await?;
        Ok(movies)
    }

    async fn movie(&self, ctx: &Context<'_>, id: String) -> Result<Movie, Error> {
        let client = &ctx.data::<SurrealConnection>()?.client;
        let movie: Movie = client.select(("movie", id)).await?;
        Ok(movie)
    }
}
