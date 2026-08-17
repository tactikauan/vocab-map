use crate::db;
use crate::models::Embedding;
use crate::schema::{embeddings, user_vocab, vocab};

use diesel::prelude::*;
use diesel::{RunQueryDsl, SelectableHelper};
use pgvector::{Vector, VectorExpressionMethods};

pub fn predict_from_word(
    word_to_predict: &str,
    count: i64,
    vocab_only: bool,
) -> Result<Vec<String>, &str> {
    let connection = &mut db::establish_connection();

    let Ok(embedding) = embeddings::table
        .filter(embeddings::word.eq(word_to_predict))
        .select(Embedding::as_select())
        .first(connection)
    else {
        return Err("Word not found");
    };

    let mut query = embeddings::table
        .left_join(vocab::table.on(embeddings::word.ilike(vocab::word)))
        .order_by(embeddings::vector.l2_distance(embedding.vector))
        .limit(count)
        .select(Embedding::as_select())
        .into_boxed();

    if vocab_only {
        query = query.filter(vocab::id.is_not_null());
    }

    let Ok(mut related_words) = query.load(connection) else {
        return Err("Could not find words");
    };

    let mut words = vec![];
    while let Some(w) = related_words.pop() {
        words.push(w.word);
    }
    words.reverse();

    Ok(words)
}

pub fn predict_from_words(
    words: Vec<String>,
    count: i64,
    vocab_only: bool,
    user_id: i32,
) -> Result<Vec<String>, &'static str> {
    let connection = &mut db::establish_connection();

    let mut vecs = vec![];

    for w in words.iter() {
        let Ok(embedding) = embeddings::table
            .filter(embeddings::word.eq(w))
            .select(Embedding::as_select())
            .first(connection)
        else {
            return Err("Word not found");
        };
        vecs.push(embedding.vector.to_vec());
    }

    let sum = vecs
        .into_iter()
        .reduce(|a, b| a.iter().zip(b.iter()).map(|(&a, &b)| a + b).collect())
        .unwrap();

    let mut query = embeddings::table
        .left_join(vocab::table.on(embeddings::word.ilike(vocab::word)))
        .left_join(
            user_vocab::table.on(vocab::id
                .eq(user_vocab::vocab)
                .and(user_vocab::user.eq(user_id))),
        )
        .order_by(embeddings::vector.l2_distance(Vector::from(sum)))
        .limit(count)
        .select(Embedding::as_select())
        .into_boxed();

    if vocab_only {
        query = query.filter(vocab::id.is_not_null());
    }

    query = query.filter(user_vocab::user.is_null());

    let Ok(mut related_words) = query.load(connection) else {
        return Err("Could not find words");
    };

    let mut words = vec![];
    while let Some(w) = related_words.pop() {
        words.push(w.word);
    }
    words.reverse();

    Ok(words)
}
