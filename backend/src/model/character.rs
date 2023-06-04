use async_graphql::*;
use serde::{Deserialize, Serialize};

use crate::{scalar::SurrealID, SurrealConnection};

use super::movie::Movie;

#[derive(Debug, Serialize, Deserialize, SimpleObject)]
#[graphql(complex)]
pub struct Character {
    pub id: Option<SurrealID>,
    pub name: String,
    pub image: Option<String>,
    pub description: Option<String>,
    #[serde(rename(deserialize = "movie"))]
    #[graphql(skip)]
    pub movie_id: SurrealID,
}

#[ComplexObject]
impl Character {
    async fn movie<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Movie, Error> {
        let client = &ctx.data::<SurrealConnection>()?.client;
        let movie: Movie = client.select(("movie", self.movie_id.0.clone().id)).await?;
        Ok(movie)
    }
}

#[derive(Default)]
pub struct CharacterQuery;

#[Object]
impl CharacterQuery {
    async fn characters(
        &self,
        ctx: &Context<'_>,
        movie_id: String,
    ) -> Result<Vec<Character>, Error> {
        let client = &ctx.data::<SurrealConnection>()?.client;
        let mut result = client
            .query(format!(
                "SELECT * FROM character WHERE movie == {};",
                movie_id
            ))
            .await?;
        let characters: Vec<Character> = result.take(0)?;
        Ok(characters)
    }

    async fn character(&self, ctx: &Context<'_>, id: String) -> Result<Character, Error> {
        let client = &ctx.data::<SurrealConnection>()?.client;
        let character: Character = client.select(("character", id)).await?;
        Ok(character)
    }
}
