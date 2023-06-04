use async_graphql::*;
use serde::{Deserialize, Serialize};

use crate::{scalar::SurrealID, SurrealConnection};

use super::{character::Character, movie::Movie, scene::Scene};

#[derive(Debug, Serialize, Deserialize, SimpleObject)]
#[graphql(complex)]
pub struct Conversation {
    pub id: Option<SurrealID>,
    pub name: String,
    #[serde(rename(deserialize = "participants"))]
    #[graphql(skip)]
    pub participants_ids: Vec<SurrealID>,
    #[serde(rename(deserialize = "scene"))]
    #[graphql(skip)]
    pub scene_id: SurrealID,
    #[serde(rename(deserialize = "movie"))]
    #[graphql(skip)]
    pub movie_id: SurrealID,
}

#[ComplexObject]
impl Conversation {
    async fn movie<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Movie, Error> {
        let client = &ctx.data::<SurrealConnection>()?.client;
        let movie: Movie = client.select(("movie", self.movie_id.0.id.clone())).await?;
        Ok(movie)
    }

    async fn participants<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Vec<super::Character>, Error> {
        let client = &ctx.data::<super::SurrealConnection>()?.client;
        let mut result = client
            .query(format!(
                "SELECT * FROM [{}];",
                self.participants_ids
                    .iter()
                    .map(|id| id.0.to_string())
                    .collect::<Vec<String>>()
                    .join(", "),
            ))
            .await?;
        let participants: Vec<Character> = result.take(0)?;
        Ok(participants)
    }

    async fn scene<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Scene, Error> {
        let client = &ctx.data::<super::SurrealConnection>()?.client;
        let scene: Scene = client.select(("scene", self.scene_id.0.id.clone())).await?;
        Ok(scene)
    }
}

#[derive(Default)]
pub struct ConversationQuery;

#[Object]
impl ConversationQuery {
    async fn conversations(
        &self,
        ctx: &Context<'_>,
        movie_id: String,
    ) -> Result<Vec<Conversation>, Error> {
        let client = &ctx.data::<SurrealConnection>()?.client;
        let mut result = client
            .query(format!(
                "SELECT * FROM conversation WHERE movie == {};",
                movie_id
            ))
            .await?;
        let conversations: Vec<Conversation> = result.take(0)?;
        Ok(conversations)
    }

    async fn conversation(&self, ctx: &Context<'_>, id: String) -> Result<Conversation, Error> {
        let client = &ctx.data::<SurrealConnection>()?.client;
        let conversation: Conversation = client.select(("conversation", id)).await?;
        Ok(conversation)
    }
}
