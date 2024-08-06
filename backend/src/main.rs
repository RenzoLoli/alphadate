mod config;
mod services;

use actix_web::{get, middleware::Logger, App, HttpServer, Responder};
use config::ServerOptions;
use env_logger::Env;
use services::EnvService;

#[get("/")]
async fn test() -> impl Responder {
    "Hola mundo"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // load env variables
    EnvService::load();

    // init logger
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // init server
    let server_opts = ServerOptions::load();
    HttpServer::new(|| App::new().wrap(Logger::default()).service(test))
        .bind((server_opts.host.clone(), server_opts.port))?
        .run()
        .await
}
