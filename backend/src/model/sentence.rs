use crate::{scalar::SurrealID, SurrealConnection};
use async_graphql::*;
use serde::{Deserialize, Serialize};

use super::{character::Character, conversation::Conversation, movie::Movie};

// given the context given in the above comemnts write the struct for sentence
#[derive(Debug, Serialize, Deserialize, SimpleObject)]
#[graphql(complex)]
pub struct Sentence {
    pub id: Option<SurrealID>,
    pub text: String,
    #[serde(rename(deserialize = "speaker"))]
    #[graphql(skip)]
    pub speaker_id: SurrealID,
    #[serde(rename(deserialize = "conversation"))]
    #[graphql(skip)]
    pub conversation_id: SurrealID,
    #[serde(rename(deserialize = "directed_to"))]
    #[graphql(skip)]
    pub directed_to_ids: Vec<SurrealID>,
    #[serde(rename(deserialize = "movie"))]
    #[graphql(skip)]
    pub movie_id: SurrealID,
}

#[ComplexObject]
impl Sentence {
    async fn speaker<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Character, Error> {
        let client = &ctx.data::<SurrealConnection>()?.client;
        let speaker: Character = client
            .select(("character", self.speaker_id.0.id.clone()))
            .await?;
        Ok(speaker)
    }

    async fn conversation<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Conversation, Error> {
        let client = &ctx.data::<SurrealConnection>()?.client;
        let conversation: Conversation = client
            .select(("conversation", self.conversation_id.0.id.clone()))
            .await?;
        Ok(conversation)
    }

    async fn directed_to<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<Character>, Error> {
        let client = &ctx.data::<SurrealConnection>()?.client;
        let mut result = client
            .query(format!(
                "SELECT * FROM [{}];",
                self.directed_to_ids
                    .iter()
                    .map(|id| id.0.to_string())
                    .collect::<Vec<String>>()
                    .join(", "),
            ))
            .await?;
        let directed_to: Vec<Character> = result.take(0)?;
        Ok(directed_to)
    }

    async fn movie<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Movie, Error> {
        let client = &ctx.data::<SurrealConnection>()?.client;
        let movie: Movie = client.select(("movie", self.movie_id.0.id.clone())).await?;
        Ok(movie)
    }
}

#[derive(Default)]
pub struct SentenceQuery;

#[Object]
impl SentenceQuery {
    async fn sentences(&self, ctx: &Context<'_>, movie_id: String) -> Result<Vec<Sentence>, Error> {
        let client = &ctx.data::<SurrealConnection>()?.client;
        let mut result = client
            .query(format!(
                "SELECT * FROM sentence WHERE movie == {};",
                movie_id
            ))
            .await?;
        let sentences: Vec<Sentence> = result.take(0)?;
        Ok(sentences)
    }

    async fn sentence(&self, ctx: &Context<'_>, id: String) -> Result<Sentence, Error> {
        let client = &ctx.data::<SurrealConnection>()?.client;
        let sentence: Sentence = client.select(("sentence", id)).await?;
        Ok(sentence)
    }
}
