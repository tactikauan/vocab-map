use crate::models::Embedding;
use crate::{db, schema};

use diesel::prelude::*;
use diesel::{RunQueryDsl, SelectableHelper};
use pgvector::VectorExpressionMethods;

pub fn predict_from_word(word_to_predict: &str, count: i64) -> Result<Vec<String>, &str> {
    use self::schema::embeddings::dsl::*;

    let connection = &mut db::establish_connection();

    let Ok(embedding) = embeddings
        .filter(word.eq(word_to_predict))
        .select(Embedding::as_select())
        .first(connection)
    else {
        return Err("Word not found");
    };

    let Ok(mut related_words) = embeddings
        .order_by(vector.l2_distance(embedding.vector))
        .limit(count)
        .select(Embedding::as_select())
        .load(connection)
    else {
        return Err("Could not find words");
    };

    let mut words = vec![];
    while let Some(w) = related_words.pop() {
        words.push(w.word);
    }
    words.reverse();

    Ok(words)
}
