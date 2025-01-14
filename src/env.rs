pub struct Env {
    pub env: String,
    pub app_name: String,
    pub app_port: u16,
    pub mongodb_uri: String,
    pub mongodb_database: String,
}

// impl Env {
//     pub fn is_development(&self) -> bool {
//         self.env == "development"
//     }
// }


impl Clone for Env {
    fn clone(&self) -> Self {
        Env {
            env: self.env.to_string(),
            app_name: self.app_name.to_string(),
            app_port: self.app_port,
            mongodb_uri: self.mongodb_uri.to_string(),
            mongodb_database: self.mongodb_database.to_string(),
        }
    }
}

pub fn load_env() -> Result<Env, String> {
    dotenv::dotenv().ok();

    let env = std::env::var("ENV").unwrap_or("development".to_string());
    let app_name = std::env::var("APP_NAME").unwrap_or("Skills App".to_string());
    
    let app_port = std::env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse::<u16>()
        .map_err(|_| "Invalid PORT value. Must be between 1 and 65535".to_string())?;

    let mongodb_uri = std::env::var("MONGODB_URI")
        .unwrap_or("mongodb://127.0.0.1:27017".to_string());
        
    let mongodb_database = std::env::var("DATABASE_NAME")
        .unwrap_or("resume-dev".to_string());

    Ok(Env {
        env,
        app_name,
        app_port,
        mongodb_uri,
        mongodb_database,
    })
}
