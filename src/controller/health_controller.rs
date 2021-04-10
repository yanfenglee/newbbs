use actix_web::{web, get};
use crate::base::resp::JsonResponse;
use crate::base::resp::RespErr::CodeError;
use crate::domain::UserDTO;
use crate::db::RB;
//use rbatis::wrapper::Wrapper;

/// config route service
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/health")
        .service(health)
        .service(err)
        .service(db)
        .service(db2)
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

#[get("/db")]
pub async fn db() -> JsonResponse {
    info!("call db health ...");

    #[sql(RB, "SELECT 1")]
    async fn test_db() -> i32 {}

    match test_db().await {
        Ok(data) => Ok(data).into(),
        Err(e) => CodeError("101".into(), e.to_string()).into(),
    }
}

#[get("/db2")]
pub async fn db2() -> JsonResponse {
    info!("call db health 2...");

    let data = RB.fetch::<i32>("", "select 2").await;

    match data {
        Ok(data) => Ok(data).into(),
        Err(e) => CodeError("101".into(), e.to_string()).into(),
    }
}
