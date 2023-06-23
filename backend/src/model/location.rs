use async_graphql::*;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use super::movie::Movie;

#[derive(Debug, Serialize, Deserialize, SimpleObject)]
#[graphql(complex)]
pub struct Location {
    pub id: i64,
    pub name: String,
    #[graphql(skip)]
    pub movie_id: i64,
}

// SQLx and async-graphql implementations for Location

#[ComplexObject]
impl Location {
    async fn movie<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Movie, Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        let movie: Movie =
            sqlx::query_as!(Movie, "SELECT * FROM movie WHERE id = $1;", self.movie_id)
                .fetch_one(pool)
                .await?;
        Ok(movie)
    }
}

// SQLx and async-graphql implementations for LocationQuery

#[derive(Default)]
pub struct LocationQuery;

#[Object]
impl LocationQuery {
    async fn locations(&self, ctx: &Context<'_>, movie_id: i64) -> Result<Vec<Location>, Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        let locations: Vec<Location> = sqlx::query_as!(
            Location,
            "SELECT * FROM location WHERE movie_id = $1;",
            movie_id
        )
        .fetch_all(pool)
        .await?;
        Ok(locations)
    }

    async fn location(&self, ctx: &Context<'_>, id: i64) -> Result<Location, Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        let location: Location =
            sqlx::query_as!(Location, "SELECT * FROM location WHERE id = $1;", id)
                .fetch_one(pool)
                .await?;
        Ok(location)
    }
}

#[derive(Default)]
pub struct LocationMutation;

#[Object]
impl LocationMutation {
    async fn create_location(
        &self,
        ctx: &Context<'_>,
        movie_id: i64,
        name: String,
    ) -> Result<Location, Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        let location: Location = sqlx::query_as!(
            Location,
            "INSERT INTO location (movie_id, name) VALUES ($1, $2) RETURNING *;",
            movie_id,
            name
        )
        .fetch_one(pool)
        .await?;
        Ok(location)
    }

    async fn update_location(
        &self,
        ctx: &Context<'_>,
        id: i64,
        name: String,
    ) -> Result<Location, Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        let location: Location = sqlx::query_as!(
            Location,
            "UPDATE location SET name = $1 WHERE id = $2 RETURNING *;",
            name,
            id
        )
        .fetch_one(pool)
        .await?;
        Ok(location)
    }

    async fn delete_location(&self, ctx: &Context<'_>, id: i64) -> Result<Location, Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        let location: Location = sqlx::query_as!(
            Location,
            "DELETE FROM location WHERE id = $1 RETURNING *;",
            id
        )
        .fetch_one(pool)
        .await?;
        Ok(location)
    }
}
