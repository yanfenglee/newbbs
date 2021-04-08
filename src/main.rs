use newbbs::settings::Settings;
use actix_web::{HttpServer, web, HttpResponse, Responder};
use newbbs::controller::user_controller;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello newbbs")
}

#[actix_web::main]
fn main() {
    let settings = Settings::new().unwrap();
    env_logger::init();
    RB.link(&settings.database.url).await.unwrap();

    HttpServer::new(|| {
        App::new().app_data(web::JsonConfig::default().limit(8 * 1024 * 1024))
            .route("/", web::get().to(index))
            .configure(user_controller::config)
    })
        .bind(&CONFIG.server_url)?
        .run()
        .await;

    // Print out our settings
    println!("{:?}", settings);
}