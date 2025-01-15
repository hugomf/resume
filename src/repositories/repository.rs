use async_trait::async_trait;
use mongodb::bson::oid::ObjectId;
use crate::errors::AppError;

#[async_trait]
pub trait Repository<'a, T> 
where
    T: Send + Sync + 'static + serde::Serialize + serde::de::DeserializeOwned + Unpin,
    'a: 'static {
    async fn create(&self, item: T) -> Result<(), AppError>;
    async fn find_all(&self) -> Result<Vec<T>, AppError>;
    async fn get(&self, id: &ObjectId) -> Result<Option<T>, AppError>;
    async fn update(&self, id: &ObjectId, item: T) -> Result<(), AppError>;
    async fn delete(&self, id: &ObjectId) -> Result<(), AppError>;
}
