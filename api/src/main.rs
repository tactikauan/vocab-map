use actix_cors::Cors;
use actix_web::{App, HttpServer};
mod controller;
mod db;
mod graph;
mod models;
mod repository;
mod schema;
mod service;
mod util;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .service(controller::auth::create_scope())
            .service(controller::embedding::create_scope())
            .service(controller::vocab::create_scope())
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
