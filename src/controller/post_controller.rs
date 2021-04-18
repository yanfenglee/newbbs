use crate::base::resp::JsonResponse;
use actix_web::web::{Query, Json};
use crate::service::post_service;
use actix_web::{web, get, post};
use crate::domain::dto::CreatePostDto;


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/post")
        .service(list)
        .service(create_post)
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

#[post("/create")]
pub async fn create_post(param: Json<CreatePostDto>) -> JsonResponse {
    post_service::create_post(param.into_inner()).await.into()
}