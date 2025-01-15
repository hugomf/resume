use mongodb::{Client, bson::doc, bson::oid::ObjectId, Collection};
use crate::models::experience::Experience;
use crate::errors::AppError;
use futures::TryStreamExt;

#[derive(Clone)]
pub struct ExperienceRepository {
    pub collection: Collection<Experience>,
}

impl ExperienceRepository {
    #[allow(dead_code)]
    pub fn new(client: &Client, db_name: &str, collection_name: &str) -> Self {
        let db = client.database(&db_name);
        let collection = db.collection(collection_name);
        ExperienceRepository { collection }
    }

    pub async fn create_experience(&self, experience: Experience) -> Result<(), AppError> {
        self.collection.insert_one(experience)
            .await
            .map_err(|e| AppError::DatabaseError(format!(
                "Failed to create experience: {}", e
            )))?;
        Ok(())
    }

    pub async fn find_all(&self) -> Result<Vec<Experience>, AppError> {
        println!("get_experiences in repo");
        let filter = doc! {};
        let mut cursor = self.collection.find(filter)
            .await
            .map_err(|e| AppError::DatabaseError(format!(
                "Failed to fetch experiences: {}", e
            )))?;
        
        let mut experiences = Vec::new();
        while let Some(experience) = cursor.try_next()
            .await
            .map_err(|e| AppError::DatabaseError(format!(
                "Failed to process experience cursor: {}", e
            )))?
        {
            experiences.push(experience);
        }
        
        if experiences.is_empty() {
            return Err(AppError::NotFound(
                "No experiences found".to_string()
            ));
        }
        
        Ok(experiences)
    }

    pub async fn get_experience(&self, id: &ObjectId) -> Result<Option<Experience>, AppError> {
        let filter = doc! { "_id": id };
        self.collection.find_one(filter)
            .await
            .map_err(|e| AppError::DatabaseError(format!(
                "Failed to fetch experience with id {}: {}", id, e
            )))
    }

    pub async fn update_experience(&self, id: &ObjectId, experience: Experience) -> Result<(), AppError> {
        let filter = doc! { "_id": id };
        self.collection.replace_one(filter, experience)
            .await
            .map_err(|e| AppError::DatabaseError(format!(
                "Failed to update experience with id {}: {}", id, e
            )))?;
        Ok(())
    }

    pub async fn delete_experience(&self, id: &ObjectId) -> Result<(), AppError> {
        let filter = doc! { "_id": id };
        self.collection.delete_one(filter)
            .await
            .map_err(|e| AppError::DatabaseError(format!(
                "Failed to delete experience with id {}: {}", id, e
            )))?;
        Ok(())
    }
}
