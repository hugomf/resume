use mongodb::bson::{oid::ObjectId, doc};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref DATE_REGEX: Regex = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
}
use serde::{Deserialize, Serialize};
use validator::Validate;
use super::skill::Skill;
use super::responsibility::Responsibility;

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct Experience {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[validate(length(min = 1, message = "Job title is required"))]
    pub job_title: String,
    
    #[validate(length(min = 1, message = "Company name is required"))]
    pub company: String,
    
    #[validate(length(min = 1, message = "Location is required"))]
    pub location: String,
    
    #[validate(regex(
        path = "DATE_REGEX",
        message = "Start date must be in YYYY-MM-DD format"
    ))]
    pub start_date: String,
    
    #[validate(regex(
        path = "DATE_REGEX",
        message = "End date must be in YYYY-MM-DD format"
    ))]
    pub end_date: String,
    #[serde(default)]  // Make responsibilities optional with default empty Vec
    pub responsibilities: Vec<Responsibility>,
    #[serde(default)]  // Make environment optional with default empty Vec
    pub environment: Vec<Skill>
}
