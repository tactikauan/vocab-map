use actix_web::{App, HttpServer};
mod controllers;
mod db;
mod models;
mod schema;
mod services;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(controllers::embeddings::create_scope()))
        .bind(("localhost", 8080))?
        .run()
        .await
}
