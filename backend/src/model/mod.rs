use async_graphql::*;
use serde::Deserialize;

use self::{
    character::{CharacterMutation, CharacterQuery},
    conversation::{ConversationMutation, ConversationQuery},
    location::{LocationMutation, LocationQuery},
    movie::{MovieMutation, MovieQuery},
    scene::{SceneMutation, SceneQuery},
    sentence::{SentenceMutation, SentenceQuery},
};

mod character;
mod conversation;
mod location;
mod movie;
mod scene;
mod sentence;

#[derive(MergedObject, Default)]
pub struct QueryRoot(
    CharacterQuery,
    ConversationQuery,
    LocationQuery,
    MovieQuery,
    SceneQuery,
    SentenceQuery,
);

#[derive(MergedObject, Default)]
pub struct MutationRoot(
    MovieMutation,
    CharacterMutation,
    ConversationMutation,
    LocationMutation,
    SceneMutation,
    SentenceMutation,
);
