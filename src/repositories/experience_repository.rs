use mongodb::{Client, bson::doc, bson::oid::ObjectId, Collection};
use crate::models::experience::Experience;
use futures::TryStreamExt;

#[derive(Clone)]
pub struct ExperienceRepository {
    collection: Collection<Experience>,
}

impl ExperienceRepository {
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

#[cfg(test)]
mod tests {
    use super::*;
    use mongodb::Client;
    use tokio_test::block_on;

    #[test]
    fn test_find_all() {
        block_on(async {
            dotenv::dotenv().ok();
            let uri = std::env::var("MONGO_URI").expect("MONGO_URI must be set");
            let db_name = std::env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");
            
            let client = Client::with_uri_str(&uri).await.expect("Failed to connect to MongoDB");
            let repo = ExperienceRepository::new(&client, &db_name, "experience");
            
            // Clean up any existing test data
            repo.collection.delete_many(doc! {}).await.unwrap();
            
            // Insert test data
            let test_experience = Experience {
                id: None,
                job_title: "Test Job Title".to_string(),
                company: "Test Company".to_string(),
                location: "Test Location".to_string(),
                start_date: "2023-01-01".to_string(),
                end_date: "2023-12-31".to_string(),
                responsibilities: vec![],
                environment: vec![],
            };
            repo.create_experience(test_experience.clone()).await.unwrap();
            
            // Test find_all
            let result = repo.find_all().await;
            assert!(result.is_ok());
            
            let experiences = result.unwrap();
            assert!(!experiences.is_empty());
            assert_eq!(experiences[0].job_title, test_experience.job_title);
            
            // Clean up
            repo.collection.delete_many(doc! {}).await.unwrap();
        });
    }
}
