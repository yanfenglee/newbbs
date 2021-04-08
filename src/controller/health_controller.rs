
use actix_web::{Responder, web, get, HttpResponse};

/// config route service
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/health")
        .service(health)
    );
}

#[get("/")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().body("SUCCESS")
}
