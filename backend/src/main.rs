mod controllers;
mod domain;
mod middlewares;
mod config;
mod router;
mod services;

use actix_web::{middleware::Logger, App, HttpServer};
use config::ServerOptions;
use env_logger::Env;
use services::EnvService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // load env variables
    EnvService::load();

    // init logger
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // init server
    let server_opts = ServerOptions::load();
    HttpServer::new(|| App::new().wrap(Logger::default()).configure(router::config))
        .bind((server_opts.host.clone(), server_opts.port))?
        .run()
        .await
}
