use newbbs::configure::settings::Settings;
use actix_web::{HttpServer, App, web, HttpResponse, Responder};
use newbbs::controller::health_controller;
use newbbs::db::RB;
use std::net::{SocketAddr};
use log::info;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello newbbs")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    info!("server start ...");

    let settings = Settings::new().unwrap();
    RB.link(&settings.db.url).await.unwrap();

    HttpServer::new(|| {
        App::new().app_data(web::JsonConfig::default().limit(8 * 1024 * 1024))
            .route("/", web::get().to(index))
            .configure(health_controller::config)
    })
        .bind(SocketAddr::from(([0, 0, 0, 0], settings.web.port)))?
        .run()
        .await?;

    info!("server end !!!");

    Ok(())
}