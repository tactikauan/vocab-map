use crate::services::embeddings;
use crate::utils::response;

use actix_web::{HttpRequest, HttpResponse, Responder, Scope, web};
use serde::Deserialize;

async fn predict(req: HttpRequest) -> impl Responder {
    let word: String = req.match_info().load().unwrap();

    #[derive(Deserialize)]
    struct Params {
        count: Option<i64>,
    }

    let params = web::Query::<Params>::from_query(req.query_string()).unwrap();
    let count = params.count.unwrap_or_else(|| 30);

    let words = match embeddings::predict_from_word(&word, count) {
        Ok(words) => words,
        Err(e) => return HttpResponse::InternalServerError().json(response::message(e)),
    };

    HttpResponse::Ok().json(words)
}

pub fn create_scope() -> Scope {
    web::scope("/embeddings").route("/predict/{word}", web::get().to(predict))
}
