use async_trait::async_trait;
use mongodb::{Client, bson::doc, bson::oid::ObjectId, Collection};
use crate::errors::AppError;
use crate::repositories::repository::Repository;
use futures::TryStreamExt;

#[derive(Clone)]
pub struct ExperienceRepository<'a, T> where T: Send + Sync {
    pub collection: Collection<T>,
    _marker: std::marker::PhantomData<&'a ()>,
}

#[async_trait]
impl<'a, T> Repository<'a, T> for ExperienceRepository<'a, T> 
where 
    T: Send + Sync + 'static + serde::Serialize + serde::de::DeserializeOwned + Unpin,
    'a: 'static {
    async fn create(&self, item: T) -> Result<(), AppError> {
        self.collection.insert_one(item)
            .await
            .map_err(|e| AppError::DatabaseError(format!(
                "Failed to create experience: {}", e
            )))?;
        Ok(())
    }

    async fn find_all(&self) -> Result<Vec<T>, AppError> {
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

    async fn get(&self, id: &ObjectId) -> Result<Option<T>, AppError> {
        let filter = doc! { "_id": id };
        self.collection.find_one(filter)
            .await
            .map_err(|e| AppError::DatabaseError(format!(
                "Failed to fetch experience with id {}: {}", id, e
            )))
    }

    async fn update(&self, id: &ObjectId, item: T) -> Result<(), AppError> {
        let filter = doc! { "_id": id };
        self.collection.replace_one(filter, item)
            .await
            .map_err(|e| AppError::DatabaseError(format!(
                "Failed to update experience with id {}: {}", id, e
            )))?;
        Ok(())
    }

    async fn delete(&self, id: &ObjectId) -> Result<(), AppError> {
        let filter = doc! { "_id": id };
        self.collection.delete_one(filter)
            .await
            .map_err(|e| AppError::DatabaseError(format!(
                "Failed to delete experience with id {}: {}", id, e
            )))?;
        Ok(())
    }

}

impl<'a, T> ExperienceRepository<'a, T> where T: Send + Sync {
    pub async fn add_responsibility(&self, id: &ObjectId, responsibility: crate::models::responsibility::Responsibility) -> Result<(), AppError> {
        let filter = doc! { "_id": id };
        let update = doc! { "$push": { "responsibilities": mongodb::bson::to_bson(&responsibility)
            .map_err(|e| AppError::DatabaseError(format!(
                "Failed to serialize responsibility: {}", e
            )))? }};
        
        self.collection.update_one(filter, update)
            .await
            .map_err(|e| AppError::DatabaseError(format!(
                "Failed to add responsibility to experience {}: {}", id, e
            )))?;
        Ok(())
    }

    pub async fn add_environment(&self, id: &ObjectId, environment: crate::models::skill::Skill) -> Result<(), AppError> {
        let filter = doc! { "_id": id };
        let update = doc! { "$push": { "environments": mongodb::bson::to_bson(&environment)
            .map_err(|e| AppError::DatabaseError(format!(
                "Failed to serialize environment: {}", e
            )))? }};
        
        self.collection.update_one(filter, update)
            .await
            .map_err(|e| AppError::DatabaseError(format!(
                "Failed to add environment to experience {}: {}", id, e
            )))?;
        Ok(())
    }
    #[allow(dead_code)]
    pub fn new(client: &Client, db_name: &str, collection_name: &str) -> ExperienceRepository<'static, T> {
        let db = client.database(db_name);
        let collection = db.collection(collection_name);
        ExperienceRepository { 
            collection,
            _marker: std::marker::PhantomData
        }
    }
}
