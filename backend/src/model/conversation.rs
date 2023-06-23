use async_graphql::*;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use super::{character::Character, movie::Movie, scene::Scene};

#[derive(Debug, Serialize, Deserialize, SimpleObject)]
#[graphql(complex)]
pub struct Conversation {
    pub id: i64,
    pub name: String,
    #[graphql(skip)]
    pub scene_id: Option<i64>,
    #[graphql(skip)]
    pub movie_id: i64,
}

// SQLx and async-graphql implementations for Conversation

#[ComplexObject]
impl Conversation {
    async fn movie<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Movie, Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        let movie: Movie =
            sqlx::query_as!(Movie, "SELECT * FROM movie WHERE id = $1;", self.movie_id)
                .fetch_one(pool)
                .await?;
        Ok(movie)
    }

    async fn participants<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<Character>, Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        let participants: Vec<Character> = sqlx::query_as!(
            Character,
            "SELECT c.* FROM character as c \
            INNER JOIN conversation_participants as cp ON c.id = cp.participant_id \
            WHERE cp.conversation_id = $1;",
            self.id
        )
        .fetch_all(pool)
        .await?;
        Ok(participants)
    }

    async fn scene<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Scene, Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        let scene: Scene =
            sqlx::query_as!(Scene, "SELECT * FROM scene WHERE id = $1;", self.scene_id)
                .fetch_one(pool)
                .await?;
        Ok(scene)
    }
}

// SQLx and async-graphql implementations for ConversationQuery

#[derive(Default)]
pub struct ConversationQuery;

#[Object]
impl ConversationQuery {
    async fn conversations(
        &self,
        ctx: &Context<'_>,
        movie_id: i64,
    ) -> Result<Vec<Conversation>, Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        let conversations: Vec<Conversation> = sqlx::query_as!(
            Conversation,
            "SELECT * FROM conversation WHERE movie_id = $1;",
            movie_id
        )
        .fetch_all(pool)
        .await?;
        Ok(conversations)
    }

    async fn conversation(&self, ctx: &Context<'_>, id: i64) -> Result<Conversation, Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        let conversation: Conversation = sqlx::query_as!(
            Conversation,
            "SELECT * FROM conversation WHERE id = $1;",
            id
        )
        .fetch_one(pool)
        .await?;
        Ok(conversation)
    }
}

#[derive(Default)]
pub struct ConversationMutation;

#[Object]
impl ConversationMutation {
    async fn create_conversation(
        &self,
        ctx: &Context<'_>,
        movie_id: i64,
        name: String,
        participant_ids: Vec<i64>,
    ) -> Result<Conversation, Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        let mut transaction = pool.begin().await?;
        let conversation: Conversation = sqlx::query_as!(
            Conversation,
            "INSERT INTO conversation (name, movie_id) VALUES ($1, $2) RETURNING *;",
            name,
            movie_id
        )
        .fetch_one(&mut transaction)
        .await?;

        for participant_id in participant_ids {
            sqlx::query!(
                "INSERT INTO conversation_participants (conversation_id, participant_id) VALUES ($1, $2);",
                conversation.id,
                participant_id
            )
            .execute(&mut transaction)
            .await?;
        }

        transaction.commit().await?;
        Ok(conversation)
    }

    async fn add_participant(
        &self,
        ctx: &Context<'_>,
        conversation_id: i64,
        participant_id: i64,
    ) -> Result<bool, Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        sqlx::query!(
            "INSERT INTO conversation_participants (conversation_id, participant_id) VALUES ($1, $2) RETURNING *;",
            conversation_id,
            participant_id
        )
        .fetch_one(pool)
        .await?;
        Ok(true)
    }

    async fn remove_participant(
        &self,
        ctx: &Context<'_>,
        conversation_id: i64,
        participant_id: i64,
    ) -> Result<bool, Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        sqlx::query!(
            "DELETE FROM conversation_participants WHERE conversation_id = $1 AND participant_id = $2 RETURNING *;",
            conversation_id,
            participant_id
        )
        .fetch_one(pool)
        .await?;
        Ok(true)
    }

    async fn delete_conversation(
        &self,
        ctx: &Context<'_>,
        conversation_id: i64,
    ) -> Result<bool, Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        sqlx::query!(
            "DELETE FROM conversation WHERE id = $1 RETURNING *;",
            conversation_id
        )
        .fetch_one(pool)
        .await?;
        Ok(true)
    }

    async fn update_conversation(
        &self,
        ctx: &Context<'_>,
        conversation_id: i64,
        name: String,
    ) -> Result<bool, Error> {
        let pool = ctx.data::<Pool<Postgres>>()?;
        sqlx::query!(
            "UPDATE conversation SET name = $1 WHERE id = $2 RETURNING *;",
            name,
            conversation_id
        )
        .fetch_one(pool)
        .await?;
        Ok(true)
    }
}
