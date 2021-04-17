use crate::base::resp::JsonResponse;
use actix_web::web::{Query};
use crate::service::post_service;
use actix_web::{web, get};


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/post")
        .service(list)
    );
}

#[derive(Debug, Deserialize)]
pub struct ListParam {
    start: i32,
    limit: i32,
}
#[get("/list")]
pub async fn list(param: Query<ListParam>) -> JsonResponse {
    post_service::list(param.start, param.limit).await.into()
}