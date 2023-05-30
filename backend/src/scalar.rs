use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, Value};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize)]
pub struct SurrealID(Thing);

#[Scalar]
impl ScalarType for SurrealID {
    fn parse(value: Value) -> InputValueResult<Self> {
        match value.to_string().split_once(":") {
            Some(tuple) => InputValueResult::Ok(SurrealID(Thing::from(tuple))),
            None => InputValueResult::Err(InputValueError::custom(
                "SurrealID must be in the form of `table:id`",
            )),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.to_string())
    }
}
