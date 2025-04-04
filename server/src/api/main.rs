use std::{env, io};
use actix_web::{http, middleware, App, HttpServer};
use crate::api::world;
use actix_cors::Cors;

pub async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .service(world::world)
    })
        .bind("127.0.0.1:9090")?
        .run()
        .await
}