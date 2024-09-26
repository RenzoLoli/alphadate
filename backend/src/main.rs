mod config;
mod controllers;
mod database;
mod domain;
mod middlewares;
mod repository;
mod router;
mod services;

use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, App, HttpServer};
use config::ServerOptions;
use database::{config_database, Connection};
use env_logger::Env;
use repository::Repositories;
use services::{
    BaseOfServices, ContextServices, EnvService, PasswordService, Services, TokenService,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // load env variables
    EnvService::load();

    // server options
    let server_opts = ServerOptions::load();

    // init logger
    env_logger::init_from_env(Env::default().default_filter_or(server_opts.log_level));

    // load database connection
    let connection = Arc::new(
        Connection::default()
            .connect(&server_opts.config_connection)
            .await
            .expect("cannot connect to database"),
    );

    // config tables
    config_database(&connection).await;

    // dependencies

    log::info!("Creating repositories");
    let repositories = Repositories::new(connection);

    log::info!("Creating services");
    let base_of_services = BaseOfServices {
        password_service: Arc::new(PasswordService::new(&server_opts.password_encryption_key)),
        token_service: Arc::new(TokenService::new(server_opts.secret.clone())),
        repositories,
    };

    let services = Arc::new(Services::new().load(base_of_services));

    // init server

    log::info!("Starting server");
    HttpServer::new(move || {
        let origins = server_opts.origins.clone();
        App::new()
            // context data
            .app_data(ContextServices::from(services.clone()))
            // middlewares
            .wrap(
                Cors::default()
                    .allowed_origin_fn(move |origin, _| {
                        origins
                            .iter()
                            .any(|allowed| origin.as_bytes().starts_with(allowed.as_bytes()))
                    })
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
                    .supports_credentials()
                    .allowed_headers(vec![
                        header::AUTHORIZATION,
                        header::ACCEPT,
                        header::CONTENT_TYPE,
                    ])
                    .max_age(3600),
            )
            .wrap(Logger::default())
            // routing
            .configure(router::config)
    })
    .bind((server_opts.host.clone(), server_opts.port))?
    .run()
    .await
}
