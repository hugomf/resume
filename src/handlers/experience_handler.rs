use actix_web::{HttpResponse, web};
use validator::Validate;
use crate::models::experience::Experience;
use crate::models::responsibility::Responsibility;
use crate::models::skill::Skill;
use crate::services::experience_service::ExperienceService;
use crate::errors::AppError;

pub async fn create_experience(
    service: web::Data<ExperienceService>,
    experience: web::Json<Experience>,
) -> Result<HttpResponse, AppError> {
    experience.0.validate().map_err(|err| {
        AppError::ValidationError(err.to_string())
    })?;
    
    service.create_experience(experience.into_inner())
        .await
        .map(|_| HttpResponse::Created().finish())
        .map_err(|err| AppError::DatabaseError(err.to_string()))
}

pub async fn get_experiences(
    service: web::Data<ExperienceService>
) -> Result<HttpResponse, AppError> {
    println!("get_experiences");
    service.get_experiences()
        .await
        .map(|experiences| HttpResponse::Ok().json(experiences))
        .map_err(|err| AppError::DatabaseError(err.to_string()))
}

pub async fn get_experience(
    service: web::Data<ExperienceService>,
    id: web::Path<String>
) -> Result<HttpResponse, AppError> {
    service.get_experience(&id)
        .await
        .map(|experience| match experience {
            Some(exp) => HttpResponse::Ok().json(exp),
            None => HttpResponse::NotFound().finish(),
        })
        .map_err(|err| AppError::DatabaseError(err.to_string()))
}

pub async fn update_experience(
    service: web::Data<ExperienceService>,
    id: web::Path<String>,
    experience: web::Json<Experience>,
) -> Result<HttpResponse, AppError> {
    experience.0.validate().map_err(|err| {
        AppError::ValidationError(err.to_string())
    })?;
    
    service.update_experience(&id, experience.into_inner())
        .await
        .map(|_| HttpResponse::Ok().finish())
        .map_err(|err| AppError::DatabaseError(err.to_string()))
}

pub async fn delete_experience(
    service: web::Data<ExperienceService>,
    id: web::Path<String>
) -> Result<HttpResponse, AppError> {
    service.delete_experience(&id)
        .await
        .map(|_| HttpResponse::NoContent().finish())
        .map_err(|err| AppError::DatabaseError(err.to_string()))
}

pub async fn add_responsibility(
    service: web::Data<ExperienceService>,
    id: web::Path<String>,
    responsibility: web::Json<Responsibility>,
) -> Result<HttpResponse, AppError> {
    responsibility.0.validate().map_err(|err| {
        AppError::ValidationError(err.to_string())
    })?;
    
    service.add_responsibility(&id, responsibility.into_inner())
        .await
        .map(|_| HttpResponse::Created().finish())
        .map_err(|err| AppError::DatabaseError(err.to_string()))
}

pub async fn add_environment(
    service: web::Data<ExperienceService>,
    id: web::Path<String>,
    environment: web::Json<Skill>,
) -> Result<HttpResponse, AppError> {
    environment.0.validate().map_err(|err| {
        AppError::ValidationError(err.to_string())
    })?;
    
    service.add_environment(&id, environment.into_inner())
        .await
        .map(|_| HttpResponse::Created().finish())
        .map_err(|err| AppError::DatabaseError(err.to_string()))
}
