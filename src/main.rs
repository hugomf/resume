use actix_web::{App, HttpServer, middleware::Logger};
use actix_web::web::Data;
use env::load_env;
use mongodb::{Client, options::ClientOptions};

use crate::api::configure_routes;
use crate::repositories::experience_repository::ExperienceRepository;
use crate::services::experience_service::ExperienceService;

mod models;
mod repositories;
mod services;
mod handlers;
mod api;
mod env;
mod errors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let env = load_env().map_err(|e| {
        eprintln!("Failed to load environment: {}", e);
        std::io::Error::new(std::io::ErrorKind::Other, e)
    })?;

    // Initialize MongoDB client
    let mongo_uri = env.mongodb_uri;
    let mongo_db = env.mongodb_database;
    let app_port = env.app_port;
    let client_options = ClientOptions::parse(&mongo_uri).await.map_err(|e| {
        eprintln!("Failed to parse MongoDB URI: {}", e);
        std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
    })?;
    
    let client = Client::with_options(client_options).map_err(|e| {
        eprintln!("Failed to create MongoDB client: {}", e);
        std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
    })?;

    // Create repository and service instances
    let experience_repo = ExperienceRepository::new(&client, &mongo_db, "experience");
    let experience_service = ExperienceService::new(experience_repo);

    println!("Server: [{}]", env.app_name);
    println!("Environment: [{}]", env.env);
    println!("MONGODB_URI: [{}]", mongo_uri);
    
    println!("Url: [http://localhost:{}]", app_port.clone());

    println!();

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(experience_service.clone()))
            .configure(configure_routes)
    })
        .bind(format!("0.0.0.0:{}", app_port))?
        .run()
        .await
}
