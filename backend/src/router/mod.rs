use actix_web::{get, web, Responder};

use crate::controllers::{auth_config, user_config};

#[get("/")]
async fn test() -> impl Responder {
    "hola mundo"
}

pub fn config(cfg: &mut web::ServiceConfig) {
    // controllers
    let auth = web::scope("/auth").configure(auth_config);
    let user = web::scope("/user").configure(user_config);

    // v1
    let api_v1 = web::scope("/api/v1").service(auth).service(user);

    // configure "/" routes
    cfg.service(api_v1);

    // test route
    cfg.service(test);
}
