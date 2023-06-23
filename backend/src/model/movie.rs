use async_graphql::*;
use serde::{Deserialize, Serialize};
use sqlx::{query_as, Pool, Postgres};

#[derive(Debug, Serialize, Deserialize, SimpleObject)]
pub struct Movie {
    pub id: i64,
    pub name: String,
}

#[derive(Default)]
pub struct MovieQuery;

#[Object]
impl MovieQuery {
    async fn movies(&self, ctx: &Context<'_>) -> Result<Vec<Movie>, Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        let movies: Vec<Movie> = query_as!(Movie, "SELECT * FROM movie;")
            .fetch_all(pool)
            .await?;
        Ok(movies)
    }

    async fn movie(&self, ctx: &Context<'_>, id: i64) -> Result<Movie, Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        let movie: Movie = query_as!(Movie, "SELECT * FROM movie WHERE id = $1;", id)
            .fetch_one(pool)
            .await?;
        Ok(movie)
    }
}

#[derive(Default)]
pub struct MovieMutation;

#[Object]
impl MovieMutation {
    async fn create_movie(&self, ctx: &Context<'_>, name: String) -> Result<Movie, Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        let movie: Movie = query_as!(
            Movie,
            "INSERT INTO movie (name) VALUES ($1) RETURNING *;",
            name
        )
        .fetch_one(pool)
        .await?;
        Ok(movie)
    }

    async fn update_movie(
        &self,
        ctx: &Context<'_>,
        id: i64,
        name: Option<String>,
    ) -> Result<Movie, Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        let movie: Movie = query_as!(
            Movie,
            "UPDATE movie SET name = COALESCE($1, name) WHERE id = $2 RETURNING *;",
            name,
            id
        )
        .fetch_one(pool)
        .await?;
        Ok(movie)
    }

    async fn delete_movie(&self, ctx: &Context<'_>, id: i64) -> Result<Movie, Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        let movie: Movie = query_as!(Movie, "DELETE FROM movie WHERE id = $1 RETURNING *;", id)
            .fetch_one(pool)
            .await?;
        Ok(movie)
    }
}
