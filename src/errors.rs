use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;

#[derive(Debug)]
#[allow(dead_code)]
pub enum AppError {
    NotFound(String),
    ValidationError(String),
    DatabaseError(String),
    InternalServerError(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            AppError::ValidationError(msg) => write!(f, "Validation Error: {}", msg),
            AppError::DatabaseError(msg) => write!(f, "Database Error: {}", msg),
            AppError::InternalServerError(msg) => write!(f, "Internal Server Error: {}", msg),
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
                },
            ),
            AppError::ValidationError(msg) => (
                actix_web::http::StatusCode::BAD_REQUEST,
                ErrorResponse {
                    error: "validation_error".to_string(),
                    message: msg.to_string(),
                },
            ),
            AppError::DatabaseError(msg) => (
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    error: "database_error".to_string(),
                    message: msg.to_string(),
                },
            ),
            AppError::InternalServerError(msg) => (
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    error: "internal_server_error".to_string(),
                    message: msg.to_string(),
                },
            ),
        };

        HttpResponse::build(status).json(error)
    }
}
