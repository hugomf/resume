use mongodb::bson::{oid::ObjectId, doc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct Responsibility {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[validate(length(min = 1, message = "Responsibility name is required"))]
    pub name: String,
    pub next: Option<ObjectId>
}
