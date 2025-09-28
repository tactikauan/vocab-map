use crate::services::embeddings;
use crate::utils::response;

use actix_web::{HttpRequest, HttpResponse, Responder, Scope, web};

async fn predict(req: HttpRequest) -> impl Responder {
    let word: String = req.match_info().load().unwrap();

    let words = match embeddings::predict_from_word(&word, 30) {
        Ok(words) => words,
        Err(e) => return HttpResponse::InternalServerError().json(response::message(e)),
    };

    HttpResponse::Ok().json(words)
}

pub fn create_scope() -> Scope {
    web::scope("/embeddings").route("/predict/{word}", web::get().to(predict))
}
