use actix_web::{http::StatusCode, test, ResponseError};
use rust_server::errors::AppError;
use mongodb::error::Error as MongoError;
use std::io::Error as IoError;

#[actix_web::test]
async fn test_not_found_error() {
    let error = AppError::NotFound("Resource not found".to_string());
    assert_eq!(format!("{}", error), "Not Found: Resource not found");
    
    assert_eq!(error.error_response().status(), StatusCode::NOT_FOUND);
    
    let app = test::init_service(
        actix_web::App::new()
            .route("/", actix_web::web::get().to({
                let error = error.clone();
                move || {
                    let error = error.clone();
                    async move { error.error_response() }
                }
            }))
    ).await;
    let req = test::TestRequest::get().uri("/").to_request();
    let service_response = test::call_service(&app, req).await;
    let body: serde_json::Value = test::read_body_json(service_response).await;
    assert_eq!(body["error"], "not_found");
    assert_eq!(body["message"], "Resource not found");
    assert_eq!(body["error_code"], "ERR_NOT_FOUND");
    assert!(body["timestamp"].as_str().is_some());
}

#[actix_web::test]
async fn test_validation_error() {
    let error = AppError::ValidationError("Invalid input".to_string());
    assert_eq!(format!("{}", error), "Validation Error: Invalid input");
    
    assert_eq!(error.error_response().status(), StatusCode::BAD_REQUEST);
    
    let app = test::init_service(
        actix_web::App::new()
            .route("/", actix_web::web::get().to({
                let error = error.clone();
                move || {
                    let error = error.clone();
                    async move { error.error_response() }
                }
            }))
    ).await;
    let req = test::TestRequest::get().uri("/").to_request();
    let service_response = test::call_service(&app, req).await;
    let body: serde_json::Value = test::read_body_json(service_response).await;
    assert_eq!(body["error"], "validation_error");
    assert_eq!(body["message"], "Invalid input");
    assert_eq!(body["error_code"], "ERR_VALIDATION");
    assert!(body["timestamp"].as_str().is_some());
}

#[actix_web::test]
async fn test_error_conversions() {
    // Test MongoDB error conversion
    let mongo_error = MongoError::from(std::io::Error::new(std::io::ErrorKind::Other, "DB error"));
    let app_error: AppError = mongo_error.into();
    assert!(matches!(app_error, AppError::DatabaseError(_)));

    // Test JSON error conversion
    let json_error = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
    let app_error: AppError = json_error.into();
    assert!(matches!(app_error, AppError::SerializationError(_)));

    // Test IO error conversion
    let io_error = IoError::new(std::io::ErrorKind::Other, "IO error");
    let app_error: AppError = io_error.into();
    assert!(matches!(app_error, AppError::IoError(_)));
}

#[actix_web::test]
async fn test_error_response_structure() {
    let error = AppError::InternalServerError("Server error".to_string());
    
    let app = test::init_service(
        actix_web::App::new()
            .route("/", actix_web::web::get().to({
                let error = error.clone();
                move || {
                    let error = error.clone();
                    async move { error.error_response() }
                }
            }))
    ).await;
    let req = test::TestRequest::get().uri("/").to_request();
    let service_response = test::call_service(&app, req).await;
    let body: serde_json::Value = test::read_body_json(service_response).await;
    
    assert_eq!(body["error"], "internal_server_error");
    assert_eq!(body["message"], "Server error");
    assert_eq!(body["error_code"], "ERR_SERVER");
    assert!(body["timestamp"].as_str().is_some());
}
