use crate::db;
use crate::db::Language;
use crate::models::Vocab;
use crate::schema::{user_vocab, vocab};

use diesel::dsl::{delete, insert_into};
use diesel::prelude::*;
use diesel::{RunQueryDsl, SelectableHelper};

pub fn insert(word: &str, lang: Language) -> Result<Vocab, &'static str> {
    let connection = &mut db::establish_connection();

    insert_into(vocab::table)
        .values((vocab::word.eq(word), vocab::lang.eq(lang)))
        .get_result(connection)
        .or(Err("Could not insert vocab"))
}

pub fn get_by_user(user_id: i32) -> Result<Vec<Vocab>, &'static str> {
    let connection = &mut db::establish_connection();

    vocab::table
        .inner_join(user_vocab::table.on(vocab::id.eq(user_vocab::vocab)))
        .filter(user_vocab::user.eq(user_id))
        .select(Vocab::as_select())
        .load(connection)
        .or(Err("Could not load vocab"))
}

pub fn get_by_word(word: &str) -> Result<Vocab, &'static str> {
    let connection = &mut db::establish_connection();

    vocab::table
        .filter(vocab::word.eq(word))
        .select(Vocab::as_select())
        .first(connection)
        .or(Err("Could not find word"))
}

pub fn get_by_words(words: &[String]) -> Result<Vec<Vocab>, &'static str> {
    let connection = &mut db::establish_connection();

    vocab::table
        .filter(vocab::word.eq_any(words))
        .select(Vocab::as_select())
        .load(connection)
        .or(Err("Could not find words"))
}

pub fn add_to_user(word_id: i32, user_id: i32) -> Result<usize, &'static str> {
    let connection = &mut db::establish_connection();

    insert_into(user_vocab::table)
        .values((user_vocab::vocab.eq(word_id), user_vocab::user.eq(user_id)))
        .execute(connection)
        .or(Err("Could not add word"))
}

pub fn delete_from_user(word_ids: &[i32], user_id: i32) -> Result<usize, &'static str> {
    let connection = &mut db::establish_connection();

    return delete(user_vocab::table)
        .filter(user_vocab::vocab.eq_any(word_ids))
        .filter(user_vocab::user.eq(user_id))
        .execute(connection)
        .or(Err("Could not delete words"));
}

pub fn search(search: &str, limit: i64) -> Result<Vec<Vocab>, &'static str> {
    let connection = &mut db::establish_connection();

    return vocab::table
        .filter(vocab::word.ilike(format!("{}%", search)))
        .limit(limit)
        .select(Vocab::as_select())
        .load(connection)
        .or(Err("Could not load vocab"));
}
