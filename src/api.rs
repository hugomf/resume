use actix_web::web;
use crate::handlers::experience_handler::{
    create_experience,
    get_experiences,
    get_experience,
    update_experience,
    delete_experience,
    add_responsibility,
    add_environment,
};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/experience")
            .route("", web::post().to(create_experience))
            .route("", web::get().to(get_experiences))
            .route("/{id}", web::get().to(get_experience))
            .route("/{id}", web::put().to(update_experience))
            .route("/{id}", web::delete().to(delete_experience))
            .route("/add-responsibility", web::post().to(add_responsibility))
            .route("/add-environment", web::post().to(add_environment))
    );
}
