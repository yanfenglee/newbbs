
use actix_web::{web, get};
use crate::base::resp::JsonResponse;
use log::info;
use crate::base::resp::RespErr::CodeError;
use crate::domain::UserDTO;

/// config route service
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/health")
        .service(health)
        .service(err)
    );
}

#[get("")]
pub async fn health() -> JsonResponse {
    //info!("call health...");

    let user = UserDTO {
        token: "xxx".into(),
        username: "newbbs".into(),
        nickname: "newbbs-baby".into(),
    };

    Ok(user).into()
}

#[get("/error")]
pub async fn err() -> JsonResponse {
    info!("call health err...");
    CodeError("101".into(), "test error".into()).into()
}
