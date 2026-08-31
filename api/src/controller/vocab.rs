use crate::service::vocab;
use crate::util::response;
use crate::util::token::get_user_id;

use actix_web::{HttpRequest, HttpResponse, Responder, Scope, web};
use serde::Deserialize;

async fn get(req: HttpRequest) -> impl Responder {
    let Some(user_id) = get_user_id(req.headers()) else {
        return HttpResponse::Unauthorized().json(response::message("Unauthorized"));
    };

    let Ok(words) = vocab::get_user(user_id) else {
        return HttpResponse::InternalServerError().json(response::message("Could not find words"));
    };

    HttpResponse::Ok().json(words)
}

async fn get_projected(req: HttpRequest) -> impl Responder {
    let Some(user_id) = get_user_id(req.headers()) else {
        return HttpResponse::Unauthorized().json(response::message("Unauthorized"));
    };

    let Ok(result) = vocab::get_user_projected(user_id) else {
        return HttpResponse::InternalServerError().json(response::message("Could not find words"));
    };

    HttpResponse::Ok().json(result)
}

async fn add(req: HttpRequest) -> impl Responder {
    let word: String = req.match_info().load().unwrap();

    let Some(user_id) = get_user_id(req.headers()) else {
        return HttpResponse::Unauthorized().json(response::message("Unauthorized"));
    };

    let Ok(result) = vocab::add_user(&word, user_id) else {
        return HttpResponse::InternalServerError().json(response::message("Could not add word"));
    };

    HttpResponse::Ok().json(response::message(result))
}

#[derive(Deserialize)]
struct Words {
    words: Vec<String>,
}

async fn add_from_words(params: web::Json<Words>, req: HttpRequest) -> impl Responder {
    let words = params.into_inner().words;

    let Some(user_id) = get_user_id(req.headers()) else {
        return HttpResponse::Unauthorized().json(response::message("Unauthorized"));
    };

    let result = match vocab::add_user_from_words(words, user_id) {
        Ok(result) => result,
        Err(e) => return HttpResponse::InternalServerError().json(response::message(e)),
    };

    HttpResponse::Ok().json(response::message(&format!("Added word: {}", result)))
}

async fn delete_words(params: web::Json<Words>, req: HttpRequest) -> impl Responder {
    let words = params.into_inner().words;

    let Some(user_id) = get_user_id(req.headers()) else {
        return HttpResponse::Unauthorized().json(response::message("Unauthorized"));
    };

    let result = match vocab::delete_user_words(words, user_id) {
        Ok(result) => result,
        Err(e) => return HttpResponse::InternalServerError().json(response::message(e)),
    };

    HttpResponse::Ok().json(response::message(result))
}

async fn search(req: HttpRequest) -> impl Responder {
    let word: String = req.match_info().load().unwrap();

    let Ok(words) = vocab::search(&word) else {
        return HttpResponse::InternalServerError().json(response::message("Could not find words"));
    };

    HttpResponse::Ok().json(words)
}

pub fn create_scope() -> Scope {
    web::scope("/vocab")
        .route("", web::get().to(get))
        .route("/projected", web::get().to(get_projected))
        .route("/add/{word}", web::put().to(add))
        .route("/add", web::post().to(add_from_words))
        .route("/search/{word}", web::get().to(search))
        .route("/delete", web::post().to(delete_words))
}
