use async_graphql::*;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use super::{location::Location, movie::Movie};

#[derive(Debug, Serialize, Deserialize, SimpleObject)]
#[graphql(complex)]
pub struct Scene {
    pub id: i64,
    pub name: String,
    #[graphql(skip)]
    pub location_id: Option<i64>,
    #[graphql(skip)]
    pub movie_id: i64,
}

#[ComplexObject]
impl Scene {
    async fn movie<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Movie, Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        let movie: Movie =
            sqlx::query_as!(Movie, "SELECT * FROM movie WHERE id = $1;", self.movie_id)
                .fetch_one(pool)
                .await?;
        Ok(movie)
    }

    async fn location<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Option<Location>, Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        match self.location_id {
            Some(location_id) => {
                let location = sqlx::query_as!(
                    Location,
                    "SELECT * FROM location WHERE id = $1;",
                    location_id
                )
                .fetch_one(pool)
                .await?;
                Ok(Some(location))
            }
            None => Ok(None),
        }
    }
}

// SQLx and async-graphql implementations for SceneQuery

#[derive(Default)]
pub struct SceneQuery;

#[Object]
impl SceneQuery {
    async fn scenes(&self, ctx: &Context<'_>, movie_id: i64) -> Result<Vec<Scene>, Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        let scenes: Vec<Scene> =
            sqlx::query_as!(Scene, "SELECT * FROM scene WHERE movie_id = $1;", movie_id)
                .fetch_all(pool)
                .await?;
        Ok(scenes)
    }

    async fn scene(&self, ctx: &Context<'_>, id: i64) -> Result<Scene, Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        let scene: Scene = sqlx::query_as!(Scene, "SELECT * FROM scene WHERE id = $1;", id)
            .fetch_one(pool)
            .await?;
        Ok(scene)
    }
}

#[derive(Default)]
pub struct SceneMutation;

#[Object]
impl SceneMutation {
    async fn add_scene(
        &self,
        ctx: &Context<'_>,
        name: String,
        location_id: Option<i64>,
        movie_id: i64,
    ) -> Result<Scene, Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        let scene: Scene = sqlx::query_as!(
            Scene,
            "INSERT INTO scene (name, location_id, movie_id) VALUES ($1, $2, $3) RETURNING *;",
            name,
            location_id,
            movie_id
        )
        .fetch_one(pool)
        .await?;
        Ok(scene)
    }

    async fn update_scene(
        &self,
        ctx: &Context<'_>,
        id: i64,
        name: String,
        location_id: Option<i64>,
        movie_id: i64,
    ) -> Result<Scene, Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        let scene: Scene = sqlx::query_as!(
            Scene,
            "UPDATE scene SET name = $1, location_id = $2, movie_id = $3 WHERE id = $4 RETURNING *;",
            name,
            location_id,
            movie_id,
            id
        )
        .fetch_one(pool)
        .await?;
        Ok(scene)
    }

    async fn delete_scene(&self, ctx: &Context<'_>, id: i64) -> Result<Scene, Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        let scene: Scene =
            sqlx::query_as!(Scene, "DELETE FROM scene WHERE id = $1 RETURNING *;", id)
                .fetch_one(pool)
                .await?;
        Ok(scene)
    }
}
