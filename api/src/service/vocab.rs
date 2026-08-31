use std::vec;

use crate::db::Language;
use crate::models::{Embedding, Vocab};
use crate::schema::{embedding, user_vocab, vocab};
use crate::{db, service};

use diesel::dsl::{delete, insert_into};
use diesel::prelude::*;
use diesel::{RunQueryDsl, SelectableHelper};
use linfa::traits::Transformer;
use linfa_tsne::TSneParams;
use ndarray::Array2;

pub fn get_user(user_id: i32) -> Result<Vec<String>, &'static str> {
    let connection = &mut db::establish_connection();

    let Ok(mut words) = vocab::table
        .inner_join(user_vocab::table.on(vocab::id.eq(user_vocab::vocab)))
        .filter(user_vocab::user.eq(user_id))
        .select(Vocab::as_select())
        .load(connection)
    else {
        return Err("Could not load vocab");
    };

    let mut result_words = vec![];
    while let Some(w) = words.pop() {
        result_words.push(w.word);
    }
    result_words.reverse();

    Ok(result_words)
}

#[derive(serde::Serialize)]
pub struct ProjectedWord {
    pub word: String,
    pub x: f32,
    pub y: f32,
}

pub fn get_user_projected(user_id: i32) -> Result<Vec<ProjectedWord>, &'static str> {
    let connection = &mut db::establish_connection();

    let Ok(words): Result<Vec<Embedding>, _> = embedding::table
        .inner_join(vocab::table.on(embedding::word.eq(vocab::word)))
        .inner_join(user_vocab::table.on(vocab::id.eq(user_vocab::vocab)))
        .filter(user_vocab::user.eq(user_id))
        .select(Embedding::as_select())
        .load(connection)
    else {
        return Err("Could not load vocab");
    };

    let word_count = words.len();

    if word_count == 0 {
        return Ok(vec![]);
    }

    let dim = 300;
    let values = words.iter().map(|x| x.vector.to_vec()).flatten().collect();

    let values: Array2<f32> = Array2::from_shape_vec((words.len(), dim), values).unwrap();

    let perplexity: f32 = if word_count > 1 {
        12.0 * (word_count as f32) / 250.0
    } else {
        0.0
    };

    let y_2d = TSneParams::embedding_size(2)
        .perplexity(perplexity)
        .approx_threshold(0.3)
        .transform(values)
        .unwrap();

    let mut result: Vec<ProjectedWord> = vec![];
    let mut y_2d_iter = y_2d.outer_iter().into_iter();
    for w in words {
        let y = y_2d_iter.next().unwrap();
        result.push(ProjectedWord {
            word: w.word,
            x: y[0],
            y: y[1],
        });
    }

    Ok(result)
}

pub fn add_user(word: &str, user_id: i32) -> Result<&'static str, &'static str> {
    let connection = &mut db::establish_connection();

    let Ok(word) = vocab::table
        .filter(vocab::word.eq(word))
        .select(Vocab::as_select())
        .first(connection)
    else {
        return Err("Could not find word");
    };

    let result = insert_into(user_vocab::table)
        .values((user_vocab::vocab.eq(word.id), user_vocab::user.eq(user_id)))
        .execute(connection);

    if result.is_err() {
        return Err("Could not add word");
    }

    Ok("Word added successfully")
}

pub fn add_user_from_words(words: Vec<String>, user_id: i32) -> Result<String, &'static str> {
    let connection = &mut db::establish_connection();

    let Ok(mut words_to_add) = service::embeddings::predict_from_words(words, 1, false, user_id)
    else {
        return Err("Could not predict word");
    };

    let word_to_add = words_to_add.pop().unwrap();

    let word = match vocab::table
        .filter(vocab::word.eq(&word_to_add))
        .select(Vocab::as_select())
        .first(connection)
    {
        Ok(word) => word,
        Err(_) => {
            let result: Vocab = insert_into(vocab::table)
                .values((vocab::word.eq(&word_to_add), vocab::lang.eq(Language::En)))
                .get_result(connection)
                .unwrap();

            result
        }
    };

    let result = insert_into(user_vocab::table)
        .values((user_vocab::vocab.eq(word.id), user_vocab::user.eq(user_id)))
        .execute(connection);

    if result.is_err() {
        return Err("Could not insert word");
    }

    Ok(word_to_add)
}

pub fn delete_user_words(words: Vec<String>, user_id: i32) -> Result<&'static str, &'static str> {
    let connection = &mut db::establish_connection();

    let result = delete(user_vocab::table)
        .filter(
            user_vocab::vocab.eq_any(
                vocab::table
                    .filter(vocab::word.eq_any(words))
                    .select(vocab::id),
            ),
        )
        .filter(user_vocab::user.eq(user_id))
        .execute(connection);

    if result.is_err() {
        return Err("Could not delete words");
    }

    Ok("Words deleted successfully")
}

pub fn search(search: &str) -> Result<Vec<String>, &'static str> {
    let connection = &mut db::establish_connection();

    let Ok(mut words) = vocab::table
        .filter(vocab::word.ilike(format!("{}%", search)))
        .limit(20)
        .select(Vocab::as_select())
        .load(connection)
    else {
        return Err("Could not load vocab");
    };

    let mut result_words = vec![];
    while let Some(w) = words.pop() {
        result_words.push(w.word);
    }
    result_words.reverse();

    Ok(result_words)
}
