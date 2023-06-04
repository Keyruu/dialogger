use async_graphql::*;
use serde::{Deserialize, Serialize};

use crate::{scalar::SurrealID, SurrealConnection};

use super::{location::Location, movie::Movie};

#[derive(Debug, Serialize, Deserialize, SimpleObject)]
#[graphql(complex)]
pub struct Scene {
    pub id: Option<SurrealID>,
    pub name: String,
    #[serde(rename(deserialize = "location"))]
    #[graphql(skip)]
    pub location_id: SurrealID,
    #[serde(rename(deserialize = "movie"))]
    #[graphql(skip)]
    pub movie_id: SurrealID,
}

#[ComplexObject]
impl Scene {
    async fn movie<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Movie, Error> {
        let client = &ctx.data::<SurrealConnection>()?.client;
        let movie: Movie = client.select(("movie", self.movie_id.0.id.clone())).await?;
        Ok(movie)
    }

    async fn location<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Location, Error> {
        let client = &ctx.data::<SurrealConnection>()?.client;
        let movie: Location = client
            .select(("location", self.location_id.0.id.clone()))
            .await?;
        Ok(movie)
    }
}

#[derive(Default)]
pub struct SceneQuery;

#[Object]
impl SceneQuery {
    async fn scenes(&self, ctx: &Context<'_>, movie_id: String) -> Result<Vec<Scene>, Error> {
        let client = &ctx.data::<SurrealConnection>()?.client;
        let mut result = client
            .query(format!("SELECT * FROM scene WHERE movie == {};", movie_id))
            .await?;
        let scenes: Vec<Scene> = result.take(0)?;
        Ok(scenes)
    }

    async fn scene(&self, ctx: &Context<'_>, id: String) -> Result<Scene, Error> {
        let client = &ctx.data::<SurrealConnection>()?.client;
        let scene: Scene = client.select(("scene", id)).await?;
        Ok(scene)
    }
}
