use async_graphql::*;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use super::movie::Movie;

#[derive(Debug, Serialize, Deserialize, SimpleObject)]
#[graphql(complex)]
pub struct Character {
    pub id: i64,
    pub name: String,
    pub image: Option<String>,
    pub description: Option<String>,
    #[graphql(skip)]
    pub movie_id: i64,
}

// SQLx and async-graphql implementations for Character

#[ComplexObject]
impl Character {
    async fn movie<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Movie, Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        let movie = sqlx::query_as!(Movie, "SELECT * FROM movie WHERE id = $1;", self.movie_id)
            .fetch_one(pool)
            .await?;
        Ok(movie)
    }
}

// SQLx and async-graphql implementations for CharacterQuery

#[derive(Default)]
pub struct CharacterQuery;

#[Object]
impl CharacterQuery {
    async fn characters(&self, ctx: &Context<'_>, movie_id: i64) -> Result<Vec<Character>, Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        let characters = sqlx::query_as!(
            Character,
            "SELECT * FROM character WHERE movie_id = $1;",
            movie_id
        )
        .fetch_all(pool)
        .await?;
        Ok(characters)
    }

    async fn character(&self, ctx: &Context<'_>, id: i64) -> Result<Character, Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        let character = sqlx::query_as!(Character, "SELECT * FROM character WHERE id = $1;", id)
            .fetch_one(pool)
            .await?;
        Ok(character)
    }
}

#[derive(Default)]
pub struct CharacterMutation;

#[Object]
impl CharacterMutation {
    async fn create_character(
        &self,
        ctx: &Context<'_>,
        movie_id: i64,
        name: String,
        image: Option<String>,
        description: Option<String>,
    ) -> Result<Character, Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        let character: Character = sqlx::query_as!(
            Character,
            "INSERT INTO character (movie_id, name, image, description) VALUES ($1, $2, $3, $4) RETURNING *;",
            movie_id,
            name,
            image,
            description
        )
        .fetch_one(pool)
        .await?;
        Ok(character)
    }

    async fn update_character(
        &self,
        ctx: &Context<'_>,
        id: i64,
        name: Option<String>,
        image: Option<String>,
        description: Option<String>,
    ) -> Result<Character, Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        let character: Character = sqlx::query_as!(
            Character,
            "UPDATE character SET name = COALESCE($2, name), image = COALESCE($3, image), description = COALESCE($4, description) WHERE id = $1 RETURNING *;",
            id,
            name,
            image,
            description
        )
        .fetch_one(pool)
        .await?;
        Ok(character)
    }

    async fn delete_character(&self, ctx: &Context<'_>, id: i64) -> Result<Character, Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        let character: Character = sqlx::query_as!(
            Character,
            "DELETE FROM character WHERE id = $1 RETURNING *;",
            id
        )
        .fetch_one(pool)
        .await?;
        Ok(character)
    }
}
