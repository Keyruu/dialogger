use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::scalar::SurrealID;

#[derive(Debug, Serialize, Deserialize, SimpleObject)]
pub struct Person {
    pub id: Option<SurrealID>,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Record {
    #[allow(dead_code)]
    id: Thing,
}
