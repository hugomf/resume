use actix_web::{HttpResponse, ResponseError};
use mongodb::error::Error as MongoError;
use serde::Serialize;
use serde_json::Error as JsonError;
use std::fmt;
use std::io::Error as IoError;
use chrono::Utc;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum AppError {
    NotFound(String),
    ValidationError(String),
    DatabaseError(String),
    InternalServerError(String),
    InvalidObjectId(String),
    SerializationError(String),
    IoError(String),
}

impl From<MongoError> for AppError {
    fn from(err: MongoError) -> Self {
        AppError::DatabaseError(format!("Database operation failed: {}", err))
    }
}

impl From<JsonError> for AppError {
    fn from(err: JsonError) -> Self {
        AppError::SerializationError(format!("Serialization failed: {}", err))
    }
}

impl From<IoError> for AppError {
    fn from(err: IoError) -> Self {
        AppError::IoError(format!("IO operation failed: {}", err))
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_code: Option<String>,
    timestamp: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            AppError::ValidationError(msg) => write!(f, "Validation Error: {}", msg),
            AppError::DatabaseError(msg) => write!(f, "Database Error: {}", msg),
            AppError::InternalServerError(msg) => write!(f, "Internal Server Error: {}", msg),
            AppError::InvalidObjectId(msg) => write!(f, "Invalid Object ID: {}", msg),
            AppError::SerializationError(msg) => write!(f, "Serialization Error: {}", msg),
            AppError::IoError(msg) => write!(f, "IO Error: {}", msg),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let (status, error) = match self {
            AppError::NotFound(msg) => (
                actix_web::http::StatusCode::NOT_FOUND,
                ErrorResponse {
                    error: "not_found".to_string(),
                    message: msg.to_string(),
                    details: None,
                    error_code: Some("ERR_NOT_FOUND".to_string()),
                    timestamp: Utc::now().to_rfc3339(),
                },
            ),
            AppError::ValidationError(msg) => (
                actix_web::http::StatusCode::BAD_REQUEST,
                ErrorResponse {
                    error: "validation_error".to_string(),
                    message: msg.to_string(),
                    details: None,
                    error_code: Some("ERR_VALIDATION".to_string()),
                    timestamp: Utc::now().to_rfc3339(),
                },
            ),
            AppError::DatabaseError(msg) => (
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    error: "database_error".to_string(),
                    message: msg.to_string(),
                    details: None,
                    error_code: Some("ERR_DATABASE".to_string()),
                    timestamp: Utc::now().to_rfc3339(),
                },
            ),
            AppError::InternalServerError(msg) => (
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    error: "internal_server_error".to_string(),
                    message: msg.to_string(),
                    details: None,
                    error_code: Some("ERR_SERVER".to_string()),
                    timestamp: Utc::now().to_rfc3339(),
                },
            ),
            AppError::InvalidObjectId(msg) => (
                actix_web::http::StatusCode::BAD_REQUEST,
                ErrorResponse {
                    error: "invalid_object_id".to_string(),
                    message: msg.to_string(),
                    details: None,
                    error_code: Some("ERR_INVALID_ID".to_string()),
                    timestamp: Utc::now().to_rfc3339(),
                },
            ),
            AppError::SerializationError(msg) => (
                actix_web::http::StatusCode::BAD_REQUEST,
                ErrorResponse {
                    error: "serialization_error".to_string(),
                    message: msg.to_string(),
                    details: None,
                    error_code: Some("ERR_SERIALIZATION".to_string()),
                    timestamp: Utc::now().to_rfc3339(),
                },
            ),
            AppError::IoError(msg) => (
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    error: "io_error".to_string(),
                    message: msg.to_string(),
                    details: None,
                    error_code: Some("ERR_IO".to_string()),
                    timestamp: Utc::now().to_rfc3339(),
                },
            ),
        };

        HttpResponse::build(status).json(error)
    }
}
