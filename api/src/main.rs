use actix_web::{App, HttpServer};
mod db;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(services::embeddings::create_scope()))
        .bind(("localhost", 8080))?
        .run()
        .await
}
