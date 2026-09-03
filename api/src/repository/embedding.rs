use crate::db;
use crate::models::Embedding;
use crate::schema::{embedding, user_vocab, vocab};

use diesel::prelude::*;
use diesel::{RunQueryDsl, SelectableHelper};
use pgvector::{Vector, VectorExpressionMethods};

pub fn get_by_word(word: &str) -> Result<Embedding, &'static str> {
    let connection = &mut db::establish_connection();

    embedding::table
        .filter(embedding::word.eq(word))
        .select(Embedding::as_select())
        .first(connection)
        .or(Err("Word not found"))
}

pub fn get_closest_words(
    vec: &[f32],
    count: i64,
    vocab_only: bool,
    user_id: Option<i32>,
) -> Result<Vec<String>, &'static str> {
    let connection = &mut db::establish_connection();

    let mut query = embedding::table
        .left_join(vocab::table.on(embedding::word.ilike(vocab::word)))
        .left_join(
            user_vocab::table.on(vocab::id
                .eq(user_vocab::vocab)
                .and(user_vocab::user.eq(user_id.unwrap_or(0)))),
        )
        .order_by(embedding::vector.l2_distance(Vector::from(vec.to_vec())))
        .limit(count)
        .select(embedding::word)
        .into_boxed();

    if vocab_only {
        query = query.filter(vocab::id.is_not_null());
    }

    if user_id.is_some() {
        query = query.filter(user_vocab::user.is_null());
    }

    query.load(connection).or(Err("Could not find words"))
}
