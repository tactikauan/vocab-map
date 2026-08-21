use crate::db::Language;
use crate::schema;

use diesel::Selectable;
use diesel::prelude::*;
use pgvector::Vector;

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::embedding)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug)]
pub struct Embedding {
    pub word: String,
    pub vector: Vector,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::vocab)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug)]
pub struct Vocab {
    pub id: i32,
    pub word: String,
    pub lang: Language,
}
