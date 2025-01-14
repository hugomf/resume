use mongodb::bson::{oid::ObjectId, doc};
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::models::category::Category;

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct Skill {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[validate(length(min = 1, message = "Skill name is required"))]
    pub name: String,
    #[validate(length(min = 1, message = "Skill description is required"))]
    pub description: String,
    pub categories: Vec<Category>,
    pub next: Option<ObjectId>
}
