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
use database::{config_database, ConfigConnection, Connection};
use env_logger::Env;
use repository::Repositories;
use services::{ContextServices, EnvService, Services};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // load env variables
    EnvService::load();

    // init logger
    env_logger::init_from_env(
        Env::default().default_filter_or(&EnvService::get_env("LOG_LEVEL").unwrap()),
    );

    // load database connection
    let connection = Arc::new(
        Connection::default()
            .connect(&ConfigConnection {
                username: &EnvService::get_env("DB_USER").unwrap(),
                password: &EnvService::get_env("DB_PASS").unwrap(),
                address: &EnvService::get_env("DB_HOST").unwrap(),
                namespace: &EnvService::get_env("DB_NAMESPACE").unwrap(),
                database: &EnvService::get_env("DB_DATABASE").unwrap(),
            })
            .await
            .expect("cannot connect to database"),
    );

    // config tables
    config_database(&connection).await;

    let repositories = Repositories::new(connection);
    let services = Arc::new(Services::new().load(repositories));

    // init server
    let server_opts = ServerOptions::load();

    HttpServer::new(move || {
        let origins = server_opts.origins.clone();
        App::new()
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
