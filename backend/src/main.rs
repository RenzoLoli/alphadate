mod config;
mod controllers;
mod database;
mod domain;
mod middlewares;
mod repository;
mod router;
mod services;

use std::sync::Arc;

use actix_web::{middleware::Logger, web, App, HttpServer};
use config::ServerOptions;
use database::{ConfigConnection, Connection};
use env_logger::Env;
use repository::{DateIdeaRepository, UserRepository};
use services::{AuthService, DateIdeaService, EnvService, Services, UserService};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // load env variables
    EnvService::load();

    // init logger
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    // load database connection
    let connection = Connection::default()
        .connect(&ConfigConnection {
            username: "root",
            password: "root",
            address: "127.0.0.1:4700",
            namespace: "alphadate",
            database: "resources",
        })
        .await
        .expect("cannot connect to database");

    connection
        .db()
        .query(
            "
                DEFINE TABLE users SCHEMAFULL;

                DEFINE FIELD username ON TABLE users TYPE string;
                DEFINE FIELD couplename ON TABLE users TYPE string;
                DEFINE FIELD email ON TABLE users TYPE string
                    ASSERT string::is::email($value);
                DEFINE FIELD password ON TABLE users TYPE string;
                DEFINE FIELD anniversary ON TABLE users TYPE string;
                DEFINE FIELD photo ON TABLE users TYPE string;
            ",
        )
        .await
        .expect("cannot create table");

    // repositories
    let user_repository = Arc::new(UserRepository::new(connection.clone()));
    let date_idea_repository = Arc::new(DateIdeaRepository::new(connection.clone()));

    // services
    let user_service = Arc::new(UserService::new(user_repository.clone()));
    let auth_service = Arc::new(AuthService::new(user_service.clone()));
    let date_idea_service = Arc::new(DateIdeaService::new(date_idea_repository.clone()));

    // context
    let services = Arc::new(Services {
        user_service,
        auth_service,
        date_idea_service
    });

    // init server
    let server_opts = ServerOptions::load();
    HttpServer::new(move || {
        App::new()
            // services
            .app_data(web::Data::from(services.clone()))
            // middlewares
            .wrap(Logger::default())
            // routing
            .configure(router::config)
    })
    .bind((server_opts.host.clone(), server_opts.port))?
    .run()
    .await
}
