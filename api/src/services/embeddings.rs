use crate::{db, services::embeddings};
use actix_web::{HttpRequest, HttpResponse, Responder, Scope, web};
use diesel::{prelude::Queryable, Selectable};

#[derive(Queryable, Selectable)]
#[diesel(table_name = embeddings)]
pub struct Embedding {
    word: str,
    vector: Vec<f32>,
}

async fn predict_from_word(req: HttpRequest) -> impl Responder {
    let connection = db::establish_connection();

    connection.

    HttpResponse::Ok().body("ok")
}

pub fn create_scope() -> Scope {
    web::scope("/embeddings").route("/predict/{word}", web::get().to(predict_from_word))
}
