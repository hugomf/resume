use mongodb::{Client, bson::doc, bson::oid::ObjectId, Collection};
use crate::models::experience::Experience;
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

    pub async fn create_experience(&self, experience: Experience) -> mongodb::error::Result<()> {
        self.collection.insert_one(experience).await?;
        Ok(())
    }

    pub async fn find_all(&self) -> mongodb::error::Result<Vec<Experience>> {
        println!("get_experiences in repo");
        let filter = doc! {};
        let mut cursor = self.collection.find(filter).await?;
        let mut experiences = Vec::new();
        while let Some(experience) = cursor.try_next().await? {
            experiences.push(experience);
        }
        Ok(experiences)
    }

    pub async fn get_experience(&self, id: &ObjectId) -> mongodb::error::Result<Option<Experience>> {
        let filter = doc! { "_id": id };
        self.collection.find_one(filter).await
    }

    pub async fn update_experience(&self, id: &ObjectId, experience: Experience) -> mongodb::error::Result<()> {
        let filter = doc! { "_id": id };
        self.collection.replace_one(filter, experience).await?;
        Ok(())
    }

    pub async fn delete_experience(&self, id: &ObjectId) -> mongodb::error::Result<()> {
        let filter = doc! { "_id": id };
        self.collection.delete_one(filter).await?;
        Ok(())
    }
}
