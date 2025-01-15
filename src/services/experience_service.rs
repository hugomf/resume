use mongodb::bson::oid::ObjectId;
use crate::models::responsibility::Responsibility;
use crate::models::skill::Skill;
use crate::repositories::experience_repository::ExperienceRepository;
use crate::repositories::repository::Repository;
use crate::errors::AppError;

#[derive(Clone)]
pub struct ExperienceService<'a, T> 
where 
    T: Send + Sync + 'static + serde::Serialize + serde::de::DeserializeOwned + Unpin,
    'a: 'static {
    repository: ExperienceRepository<'a, T>,
}

impl<'a, T> ExperienceService<'a, T> 
where 
    T: Send + Sync + 'static + serde::Serialize + serde::de::DeserializeOwned + Unpin,
    'a: 'static {
    #[allow(dead_code)]
    pub fn new(repository: ExperienceRepository<'a, T>) -> Self {
        Self { repository }
    }

    pub async fn create_experience(&self, experience: T) -> Result<(), AppError> {
        self.repository.create(experience).await
    }

    pub async fn get_experiences(&self) -> Result<Vec<T>, AppError> {
        println!("get_experiences in service");
        self.repository.find_all().await
    }

    pub async fn get_experience(&self, id: &str) -> Result<Option<T>, AppError> {
        let object_id = ObjectId::parse_str(id).map_err(|e| AppError::InvalidObjectId(e.to_string()))?;
        self.repository.get(&object_id).await
    }

    pub async fn update_experience(&self, id: &str, experience: T) -> Result<(), AppError> {
        let object_id = ObjectId::parse_str(id).map_err(|e| AppError::InvalidObjectId(e.to_string()))?;
        self.repository.update(&object_id, experience).await
    }

    pub async fn delete_experience(&self, id: &str) -> Result<(), AppError> {
        let object_id = ObjectId::parse_str(id).map_err(|e| AppError::InvalidObjectId(e.to_string()))?;
        self.repository.delete(&object_id).await
    }

    pub async fn add_responsibility(&self, id: &str, responsibility: Responsibility) -> Result<(), AppError> {
        let object_id = ObjectId::parse_str(id).map_err(|e| AppError::InvalidObjectId(e.to_string()))?;
        self.repository.add_responsibility(&object_id, responsibility).await
    }

    pub async fn add_environment(&self, id: &str, environment: Skill) -> Result<(), AppError> {
        let object_id = ObjectId::parse_str(id).map_err(|e| AppError::InvalidObjectId(e.to_string()))?;
        self.repository.add_environment(&object_id, environment).await
    }
}
