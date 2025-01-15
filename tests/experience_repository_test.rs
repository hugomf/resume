use mongodb::{Client, bson::doc};
use tokio_test::block_on;
use rust_server::repositories::experience_repository::ExperienceRepository;
use rust_server::repositories::repository::Repository;
use rust_server::models::experience::Experience;

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
        repo.create(test_experience.clone()).await.unwrap();
        
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
