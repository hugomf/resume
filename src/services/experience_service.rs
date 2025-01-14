use mongodb::bson::oid::ObjectId;
use crate::models::experience::Experience;
use crate::models::responsibility::Responsibility;

use crate::models::skill::Skill;
use crate::repositories::experience_repository::ExperienceRepository;


#[derive(Clone)]
pub struct ExperienceService {
    repository: ExperienceRepository,
}

impl ExperienceService {
    pub fn new(repository: ExperienceRepository) -> Self {
        Self { repository }
    }

    pub async fn create_experience(&self, experience: Experience) -> Result<(), String> {
        self.repository.create_experience(experience).await.map_err(|e| e.to_string())
    }

    pub async fn get_experiences(&self) -> Result<Vec<Experience>, String> {
        println!("get_experiences in service");
        self.repository.find_all().await.map_err(|e| e.to_string())
    }

    pub async fn get_experience(&self, id: &str) -> Result<Option<Experience>, String> {
        let object_id = ObjectId::parse_str(id).map_err(|e| e.to_string())?;
        self.repository.get_experience(&object_id).await.map_err(|e| e.to_string())
    }

    pub async fn update_experience(&self, id: &str, experience: Experience) -> Result<(), String> {
        let object_id = ObjectId::parse_str(id).map_err(|e| e.to_string())?;
        self.repository.update_experience(&object_id, experience).await.map_err(|e| e.to_string())
    }

    pub async fn delete_experience(&self, id: &str) -> Result<(), String> {
        let object_id = ObjectId::parse_str(id).map_err(|e| e.to_string())?;
        self.repository.delete_experience(&object_id).await.map_err(|e| e.to_string())
    }

    pub async fn add_responsibility(&self, id:  &str, new_responsibility: Responsibility) -> Result<(), String> {
        let object_id = ObjectId::parse_str(id).map_err(|e| e.to_string())?;
        match self.repository.get_experience(&object_id).await {
            Ok(Some(mut experience)) => {
                experience.responsibilities.push(new_responsibility);
                return self.repository.update_experience(&object_id, experience).await.map_err(|e| e.to_string());
            },
            Ok(None) => {
                return Err(format!("Experience with ID '{}' not found", id));
            },
            Err(err) => {
                return Err(format!("Error retrieving experience: {}", err));
            }
        }
    }

    pub async fn add_environment(&self, id:  &str, new_environment: Skill) -> Result<(), String> {
        let object_id = ObjectId::parse_str(id).map_err(|e| e.to_string())?;
        match self.repository.get_experience(&object_id).await {
            Ok(Some(mut experience)) => {
                experience.environment.push(new_environment);
               return self.repository.update_experience(&object_id, experience).await.map_err(|e| e.to_string());
            },
            Ok(None) => {
                return Err(format!("Experience with ID '{}' not found", id));
            },
            Err(err) => {
                return Err(format!("Error retrieving experience: {}", err));
            }
        }
    }

}
