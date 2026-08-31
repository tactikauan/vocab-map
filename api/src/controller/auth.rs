use crate::service::users;
use crate::util::response;

use actix_web::{HttpResponse, Responder, Scope, web};
use serde::Deserialize;

#[derive(Deserialize)]
struct AuthParams {
    username: String,
    password: String,
}

async fn auth(params: web::Json<AuthParams>) -> impl Responder {
    let token = match users::auth(&params.username, &params.password) {
        Ok(token) => token,
        Err(e) => return HttpResponse::InternalServerError().json(response::message(e)),
    };

    HttpResponse::Ok().json(token)
}

pub fn create_scope() -> Scope {
    web::scope("/auth").route("", web::post().to(auth))
}
