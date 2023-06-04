use async_graphql::*;
use serde::Deserialize;
use surrealdb::sql::Thing;

use crate::SurrealConnection;

use self::{
    character::{Character, CharacterQuery},
    conversation::ConversationQuery,
    location::LocationQuery,
    movie::MovieQuery,
    scene::SceneQuery,
    sentence::SentenceQuery,
};

mod character;
mod conversation;
mod location;
mod movie;
mod scene;
mod sentence;

#[derive(Debug, Deserialize)]
pub struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[derive(MergedObject, Default)]
pub struct QueryRoot(
    CharacterQuery,
    ConversationQuery,
    LocationQuery,
    MovieQuery,
    SceneQuery,
    SentenceQuery,
);
