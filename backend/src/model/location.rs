use async_graphql::*;
use serde::{Deserialize, Serialize};

use crate::{scalar::SurrealID, SurrealConnection};

use super::movie::Movie;

#[derive(Debug, Serialize, Deserialize, SimpleObject)]
#[graphql(complex)]
pub struct Location {
    pub id: Option<SurrealID>,
    pub name: String,
    #[serde(rename(deserialize = "movie"))]
    #[graphql(skip)]
    pub movie_id: SurrealID,
}

#[ComplexObject]
impl Location {
    async fn movie<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Movie, Error> {
        let client = &ctx.data::<SurrealConnection>()?.client;
        let movie: Movie = client.select(("movie", self.movie_id.0.id.clone())).await?;
        Ok(movie)
    }
}

#[derive(Default)]
pub struct LocationQuery;

#[Object]
impl LocationQuery {
    async fn locations(&self, ctx: &Context<'_>, movie_id: String) -> Result<Vec<Location>, Error> {
        let client = &ctx.data::<SurrealConnection>()?.client;
        let mut result = client
            .query(format!(
                "SELECT * FROM location WHERE movie == {};",
                movie_id
            ))
            .await?;
        let locations: Vec<Location> = result.take(0)?;
        Ok(locations)
    }

    async fn location(&self, ctx: &Context<'_>, id: String) -> Result<Location, Error> {
        let client = &ctx.data::<SurrealConnection>()?.client;
        let location: Location = client.select(("location", id)).await?;
        Ok(location)
    }
}
