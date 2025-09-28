use crate::schema;

use diesel::Selectable;
use diesel::prelude::*;
use pgvector::Vector;

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::embeddings)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug)]
pub struct Embedding {
    pub word: String,
    pub vector: Vector,
}
