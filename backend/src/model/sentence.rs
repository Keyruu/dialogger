use async_graphql::*;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use super::{character::Character, conversation::Conversation, movie::Movie};

// given the context given in the above comemnts write the struct for sentence
#[derive(Debug, Serialize, Deserialize, SimpleObject)]
#[graphql(complex)]
pub struct Sentence {
    pub id: i64,
    pub text: String,
    #[graphql(skip)]
    pub speaker_id: Option<i64>,
    #[graphql(skip)]
    pub conversation_id: Option<i64>,
    #[graphql(skip)]
    pub movie_id: i64,
}

// SQLx and async-graphql implementations for Sentence

#[ComplexObject]
impl Sentence {
    async fn speaker<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Character, Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        let speaker: Character = sqlx::query_as!(
            Character,
            "SELECT * FROM character WHERE id = $1;",
            self.speaker_id
        )
        .fetch_one(pool)
        .await?;
        Ok(speaker)
    }

    async fn conversation<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Conversation, Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        let conversation: Conversation = sqlx::query_as!(
            Conversation,
            "SELECT * FROM conversation WHERE id = $1;",
            self.conversation_id
        )
        .fetch_one(pool)
        .await?;
        Ok(conversation)
    }

    async fn directed_to<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<Character>, Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        let directed_to: Vec<Character> = sqlx::query_as!(
            Character,
            "SELECT c.* FROM character as c \
            INNER JOIN sentence_directed_to as dt ON c.id = dt.directed_to_id \
            WHERE dt.sentence_id = $1;",
            self.id
        )
        .fetch_all(pool)
        .await?;
        Ok(directed_to)
    }

    async fn movie<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Movie, Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        let movie: Movie =
            sqlx::query_as!(Movie, "SELECT * FROM movie WHERE id = $1;", self.movie_id)
                .fetch_one(pool)
                .await?;
        Ok(movie)
    }
}

// SQLx and async-graphql implementations for SentenceQuery

#[derive(Default)]
pub struct SentenceQuery;

#[Object]
impl SentenceQuery {
    async fn sentences(&self, ctx: &Context<'_>, movie_id: i64) -> Result<Vec<Sentence>, Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        let sentences: Vec<Sentence> = sqlx::query_as!(
            Sentence,
            "SELECT * FROM sentence WHERE movie_id = $1;",
            movie_id
        )
        .fetch_all(pool)
        .await?;
        Ok(sentences)
    }

    async fn sentence(&self, ctx: &Context<'_>, id: i64) -> Result<Sentence, Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        let sentence: Sentence =
            sqlx::query_as!(Sentence, "SELECT * FROM sentence WHERE id = $1;", id)
                .fetch_one(pool)
                .await?;
        Ok(sentence)
    }
}

#[derive(Default)]
pub struct SentenceMutation;

#[Object]
impl SentenceMutation {
    async fn create_sentence(
        &self,
        ctx: &Context<'_>,
        text: String,
        speaker_id: Option<i64>,
        conversation_id: Option<i64>,
        movie_id: i64,
        directed_to: Vec<i64>,
    ) -> Result<Sentence, Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        let transaction = pool.begin().await?;
        let sentence: Sentence = sqlx::query_as!(
            Sentence,
            "INSERT INTO sentence (text, speaker_id, conversation_id, movie_id) VALUES ($1, $2, $3, $4) RETURNING *;",
            text,
            speaker_id,
            conversation_id,
            movie_id
        )
        .fetch_one(pool)
        .await?;

        for directed_to_id in directed_to {
            sqlx::query!(
                "INSERT INTO sentence_directed_to (sentence_id, directed_to_id) VALUES ($1, $2);",
                sentence.id,
                directed_to_id
            )
            .execute(pool)
            .await?;
        }

        transaction.commit().await?;

        Ok(sentence)
    }

    async fn update_sentence(
        &self,
        ctx: &Context<'_>,
        id: i64,
        text: Option<String>,
        speaker_id: Option<i64>,
        conversation_id: Option<i64>,
        movie_id: Option<i64>,
    ) -> Result<Sentence, Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        let sentence: Sentence = sqlx::query_as!(
            Sentence,
            "UPDATE sentence SET text = COALESCE($1, text), speaker_id = COALESCE($2, speaker_id), conversation_id = COALESCE($3, conversation_id), movie_id = COALESCE($4, movie_id) WHERE id = $5 RETURNING *;",
            text,
            speaker_id,
            conversation_id,
            movie_id,
            id
        )
        .fetch_one(pool)
        .await?;
        Ok(sentence)
    }

    async fn delete_sentence(&self, ctx: &Context<'_>, id: i64) -> Result<Sentence, Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        let sentence: Sentence = sqlx::query_as!(
            Sentence,
            "DELETE FROM sentence WHERE id = $1 RETURNING *;",
            id
        )
        .fetch_one(pool)
        .await?;
        Ok(sentence)
    }

    async fn create_directed_to(
        &self,
        ctx: &Context<'_>,
        sentence_id: i64,
        directed_to_id: i64,
    ) -> Result<bool, Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        sqlx::query!(
            "INSERT INTO sentence_directed_to (sentence_id, directed_to_id) VALUES ($1, $2) RETURNING *;",
            sentence_id,
            directed_to_id
        )
        .fetch_one(pool)
        .await?;
        Ok(true)
    }

    async fn delete_directed_to(
        &self,
        ctx: &Context<'_>,
        sentence_id: i64,
        directed_to_id: i64,
    ) -> Result<bool, Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        sqlx::query!(
            "DELETE FROM sentence_directed_to WHERE sentence_id = $1 AND directed_to_id = $2 RETURNING *;",
            sentence_id,
            directed_to_id
        )
        .fetch_one(pool)
        .await?;
        Ok(true)
    }
}
